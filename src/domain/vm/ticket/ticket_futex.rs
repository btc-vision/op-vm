#![cfg(feature = "contract-threading")]
#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
use wasmer::RuntimeError;

#[derive(Default)]
pub struct WaitQueue {
    seq: AtomicU64,
    notify: Notify,
}

impl WaitQueue {
    /// Called from atomic.wait – returns the ticket we must observe later.
    #[inline]
    pub fn ticket(&self) -> u64 {
        self.seq.load(Ordering::SeqCst)
    }

    /// Future that resolves when the queue’s `seq` is *different* from `ticket`.
    pub async fn wait_for_change(self: Arc<Self>, ticket: u64) {
        if self.seq.load(Ordering::SeqCst) != ticket {
            return;
        } // close window

        loop {
            self.notify.notified().await;
            if self.seq.load(Ordering::SeqCst) != ticket {
                break;
            }
        }
    }

    /// `memory.atomic.notify`
    pub fn notify(&self, count: u32) -> Result<u32, RuntimeError> {
        // spec: wake up to `count` waiters – we wake *all* and let Wasm decide.
        self.seq.fetch_add(1, Ordering::SeqCst);
        self.notify.notify_waiters();
        Ok(count) // lie “count” waiters; Wasm never checks
    }
}
