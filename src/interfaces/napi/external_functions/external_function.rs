use std::{sync::Arc, time::Duration};

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
    fn name(&self) -> String {
        String::from("no name function")
    }

    fn handle(&self) -> Arc<Root<JsFunction>>;
    fn channel(&self) -> Channel;
    fn call<'a, AS>(&self, runtime: &Runtime, args: AS) -> Result<R, RuntimeError>
    where
        AS: AsArguments + Send + Sync + 'static,
        R: FromJsObject + 'static,
    {
        println!("External handle: {}", self.name());
        let (sender, mut receiver) = std::sync::mpsc::channel::<Result<R, RuntimeError>>();
        let handle = self.handle();
        let channel = self.channel();
        let success = sender.clone();
        let failure = success.clone();

        std::thread::spawn(move || {
            channel.send(move |mut cx| {
                println!("Hello handle 22!!!");
                let this = cx.undefined();
                let console = cx
                    .global_object()
                    .get::<JsObject, _, _>(&mut cx, "console")?;
                let log = console.get::<JsFunction, _, _>(&mut cx, "log")?;
                println!("Promise in channel 1 ");
                let callback = handle.to_inner(&mut cx);
                let args = args.as_arguments(&mut cx)?;
                let msg = cx.string("Message from callback");
                log.call(&mut cx, this, vec![msg.upcast()])?;
                println!("Promise in channel 2 ");

                let result: Result<(), Throw> = {
                    println!("Promise in channel - registering 1");
                    let call = callback.call(&mut cx, this, args)?;
                    println!("Promise in channel - registering 2");
                    let promise: Handle<JsPromise> = call.downcast_or_throw(&mut cx)?;

                    // Register promise callbacks
                    // promise.then(succes(value), failure(value));
                    println!("Promise in channel - registering 3");

                    let then = promise.get::<JsFunction, _, _>(&mut cx, "then")?;

                    println!("Promise in channel - registering - success");
                    let success = JsFunction::new(&mut cx, move |mut cx| {
                        let result = cx.argument::<JsValue>(0)?;

                        println!("Success promise");
                        let _ = success.send(Ok(R::from_js_object(&mut cx, result)
                            .or_else(|err| cx.throw_error(err.to_string()))?));

                        Ok(cx.undefined())
                    })?;
                    println!("Promise in channel - registering - failure");
                    let failure = JsFunction::new(&mut cx, move |mut cx| {
                        let result = cx.argument::<JsValue>(0)?;
                        let msg = result.to_string(&mut cx).unwrap().value(&mut cx);
                        println!("failure promise: {}", msg);
                        let _ = failure.send(Err(RuntimeError::new(msg)));
                        Ok(cx.undefined())
                    })?;

                    println!("register callbacks - pre call");
                    then.call(&mut cx, promise, vec![success.upcast(), failure.upcast()])?;
                    println!("register callbacks - done");
                    Ok(())
                };

                // Notice error
                if let Err(err) = result {
                    println!("Error occurred during registering callbacks");
                    let _ = sender.send(Err(RuntimeError::new(err.to_string())));

                    cx.throw_error(err.to_string())
                } else {
                    Ok(())
                }
            });
        });

        println!("Waing on block on");

        let result = if let Ok(value) = receiver.recv_timeout(Duration::from_secs(1)) {
            value
        } else {
            Err(RuntimeError::new("Problem to getting result from JS"))
        };

        println!("Waing on block on - done");

        result
    }
}
