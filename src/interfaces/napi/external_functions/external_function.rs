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
    fn from_js_object<'a, C>(_cx: &mut C, _obj: Handle<'a, JsValue>) -> anyhow::Result<Self>
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
    #[allow(dead_code)]
    fn name(&self) -> String {
        String::from("no name function")
    }

    fn handle(&self) -> Arc<Root<JsFunction>>;
    fn channel(&self) -> Channel;
    fn call<'a, AS>(&self, _runtime: &Runtime, args: AS) -> Result<R, RuntimeError>
    where
        AS: AsArguments + Send + Sync + 'static,
        R: FromJsObject + 'static,
    {
        let (sender, receiver) = std::sync::mpsc::channel::<Result<R, RuntimeError>>();
        let handle = self.handle();
        let channel = self.channel();
        let success = sender.clone();
        let failure = success.clone();

        channel.send(move |mut cx| {
            let this = cx.undefined();
            let callback = handle.to_inner(&mut cx);
            let args = args.as_arguments(&mut cx)?;

            let result: Result<(), Throw> = {
                let call = callback.call(&mut cx, this, args)?;
                let promise: Handle<JsPromise> = call.downcast_or_throw(&mut cx)?;
                let success = JsFunction::new(&mut cx, move |mut cx| {
                    let result = cx.argument::<JsValue>(0)?;
                    let _ = success.send(Ok(R::from_js_object(&mut cx, result)
                        .or_else(|err| cx.throw_error(err.to_string()))?));

                    Ok(cx.undefined())
                })?;

                let failure = JsFunction::new(&mut cx, move |mut cx| {
                    let result = cx.argument::<JsError>(0)?;
                    let msg = result.to_string(&mut cx).unwrap().value(&mut cx);
                    let _ = failure.send(Err(RuntimeError::new(msg)));
                    Ok(cx.undefined())
                })?;

                let then = promise.get::<JsFunction, _, _>(&mut cx, "then")?;
                then.call(&mut cx, promise, vec![success.upcast(), failure.upcast()])?;
                Ok(())
            };

            // Notice error
            if let Err(err) = result {
                let _ = sender.send(Err(RuntimeError::new(err.to_string())));

                cx.throw_error(err.to_string())
            } else {
                Ok(())
            }
        });

        let msg = receiver.recv();

        match msg {
            Ok(Ok(result)) => Ok(result),
            Ok(Err(err)) => Err(err),
            Err(err) => Err(RuntimeError::new(err.to_string())),
        }
    }
}
