use crate::JS_THREAD;
use async_trait::async_trait;
use neon::{prelude::*, types::buffer::TypedArray};
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use wasmer::RuntimeError;

pub trait FromJsObject: Sized {
    fn from_js_object<'a, C>(cx: &mut C, v: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>;
}

impl FromJsObject for Vec<u8> {
    fn from_js_object<'a, C>(cx: &mut C, v: Handle<'a, JsValue>) -> anyhow::Result<Self>
    where
        C: Context<'a>,
    {
        let obj = v
            .downcast_or_throw::<JsBuffer, _>(cx)
            .or_else(|err| Err(anyhow::anyhow!(err.to_string())))?;

        Ok(obj.as_slice(cx).to_vec())
    }
}

impl FromJsObject for () {
    fn from_js_object<'a, C>(_cx: &mut C, _v: Handle<'a, JsValue>) -> anyhow::Result<Self>
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
#[async_trait]
pub trait ExternalFunction<R>
where
    R: FromJsObject + Send + 'static,
    Self: Send + Sync,
{
    /// The rooted JS function you want to invoke.
    fn handle(&self) -> Arc<Root<JsFunction>>;

    /// The Neon channel bound to the V8 main thread.
    fn channel(&self) -> Channel;

    /* --------------------- non-blocking helper -------------------------- */
    async fn call_async<AS>(&self, args: AS) -> Result<R, RuntimeError>
    where
        AS: AsArguments + Send + Sync + 'static,
    {
        let (tx, mut rx) = mpsc::unbounded_channel::<Result<R, RuntimeError>>();

        let js_func = self.handle();
        let channel = self.channel();
        let args = Arc::new(args);

        println!("Calling JS function asynchronously");

        channel.send(move |mut cx| {
            println!("Calling JS function");

            let this = cx.undefined();
            let callback = js_func.to_inner(&mut cx);

            // Rust → JS argument conversion
            let js_args = match args.as_arguments(&mut cx) {
                Ok(v) => v,
                Err(e) => {
                    println!("JS function args error: {}", e);
                    tx.send(Err(RuntimeError::new(e.to_string()))).ok();
                    return Ok(());
                }
            };

            // Invoke and down-cast to Promise
            let promise: Handle<JsPromise> = callback
                .call(&mut cx, this, js_args)?
                .downcast_or_throw(&mut cx)?;

            let tx_ok = tx.clone();
            let on_fulfilled = JsFunction::new(&mut cx, move |mut cx| {
                let v = cx.argument::<JsValue>(0)?;
                let parsed =
                    R::from_js_object(&mut cx, v).or_else(|e| cx.throw_error(e.to_string()))?;

                println!("Parsed JS value");

                tx_ok.send(Ok(parsed)).ok();
                Ok(cx.undefined())
            })?;

            let on_rejected = JsFunction::new(&mut cx, move |mut cx| {
                let e = cx.argument::<JsValue>(0)?;
                let msg = e.to_string(&mut cx)?.value(&mut cx);
                println!("Rejected JS value: {}", msg);

                tx.send(Err(RuntimeError::new(msg))).ok();
                Ok(cx.undefined())
            })?;

            promise
                .method(&mut cx, "then")?
                .arg(on_fulfilled)?
                .arg(on_rejected)?
                .exec()?;

            Ok(())
        });

        rx.recv()
            .await
            .unwrap_or_else(|| Err(RuntimeError::new("JavaScript promise dropped")))
    }

    fn call_blocking<AS>(&self, rt: &Runtime, args: AS) -> Result<R, RuntimeError>
    where
        AS: AsArguments + Send + Sync + 'static,
        Self: Clone + Send + Sync + 'static,
    {
        if thread::current().id() == *JS_THREAD.get().unwrap() {
            return Err(RuntimeError::new("call_blocking may not run on JS thread"));
        }

        if tokio::runtime::Handle::try_current().is_ok() {
            let this = self.clone();
            let handle = rt.handle().clone();
            return thread::spawn(move || handle.block_on(this.call_async(args)))
                .join()
                .unwrap_or_else(|_| Err(RuntimeError::new("blocking helper panicked")));
        }

        rt.block_on(self.call_async(args))
    }
}
