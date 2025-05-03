use std::sync::Arc;

use neon::{prelude::*, result::Throw, types::buffer::TypedArray};
use tokio::runtime::Runtime;
use wasmer::RuntimeError;

pub trait FromJsObject: Sized {
    fn from_js_object<'a, C>(cx: &mut C, obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>;
}

impl FromJsObject for Vec<u8> {
    fn from_js_object<'a, C>(cx: &mut C, obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>,
    {
        let obj = obj
            .downcast_or_throw::<JsBuffer, _>(cx)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        Ok(obj.as_slice(cx).to_vec())
    }
}

impl FromJsObject for () {
    fn from_js_object<'a, C>(cx: &mut C, obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>,
    {
        Ok(())
    }
}

pub trait AsArguments {
    fn as_arguments<'a, C>(&self, cx: &mut C) -> NeonResult<Vec<Handle<'a, JsValue>>>
    where
        C: Context<'a>;
}

pub trait ExternalFunction<R: Sized + Send + Sync> {
    fn handle(&self) -> Arc<Root<JsFunction>>;
    fn channel(&self) -> Channel;
    fn call<'a, AS>(&self, runtime: &Runtime, args: AS) -> Result<R, RuntimeError>
    where
        AS: AsArguments + Send + Sync + 'static,
        R: FromJsObject + 'static,
    {
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<Result<R, RuntimeError>>(1);
        let handle = self.handle();
        let channel = self.channel();
        let success = sender.clone();
        let failure = success.clone();

        channel.send(move |mut cx| {
            let callback = handle.to_inner(&mut cx);
            let args = args.as_arguments(&mut cx)?;
            let result: Result<(), Throw> = {
                let this = cx.undefined();
                let call = callback.call(&mut cx, this, args)?;
                let promise: Handle<JsPromise> = call.downcast_or_throw(&mut cx)?;

                // Register promise callbacks
                // promise.then(succes(value), failure(value));
                let then = promise.get::<JsFunction, _, _>(&mut cx, "then")?;

                let success = JsFunction::new(&mut cx, move |mut cx| {
                    let result = cx.argument::<JsValue>(0)?;

                    success
                        .try_send(Ok(R::from_js_object(&mut cx, result)
                            .or_else(|err| cx.throw_error(err.to_string()))?))
                        .unwrap();

                    Ok(cx.undefined())
                })?;
                let failure = JsFunction::new(&mut cx, move |mut cx| {
                    let result = cx.argument::<JsValue>(0)?;
                    let msg = result.to_string(&mut cx).unwrap().value(&mut cx);
                    failure.try_send(Err(RuntimeError::new(msg))).unwrap();
                    Ok(cx.undefined())
                })?;

                then.call(&mut cx, promise, vec![success.upcast(), failure.upcast()])?;

                Ok(())
            };

            if let Err(err) = result {
                sender
                    .try_send(Err(RuntimeError::new(err.to_string())))
                    .unwrap();

                cx.throw_error(err.to_string())
            } else {
                Ok(())
            }
        });

        if let Some(value) = runtime.block_on(receiver.recv()) {
            value
        } else {
            Err(RuntimeError::new("Problem to getting result from JS"))
        }
    }
}
