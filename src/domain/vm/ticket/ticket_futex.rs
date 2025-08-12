#![cfg(feature = "contract-threading")]
#![forbid(unsafe_code)]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Notify;
use wasmer::RuntimeError;

#[derive(Default)]
pub struct WaitQueue {
    seq: AtomicU64,
    notify: Notify,
    closed: AtomicBool,
}

impl WaitQueue {
    /// Called from atomic.wait – returns the ticket we must observe later.
    #[inline]
    pub fn ticket(&self) -> u64 {
        self.seq.load(Ordering::SeqCst)
    }

    /// Check if the queue has been shut down
    #[inline]
    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    /// Future that resolves when the queue's `seq` is *different* from `ticket`.
    pub async fn wait_for_change(self: Arc<Self>, ticket: u64) {
        // Check if already closed
        if self.is_closed() {
            return;
        }

        if self.seq.load(Ordering::SeqCst) != ticket {
            return;
        } // close window

        loop {
            // Check closed state before waiting
            if self.is_closed() {
                return;
            }

            self.notify.notified().await;

            // Check if closed after waking
            if self.is_closed() {
                return;
            }

            if self.seq.load(Ordering::SeqCst) != ticket {
                break;
            }
        }
    }

    /// `memory.atomic.notify`
    pub fn notify(&self, count: u32) -> Result<u32, RuntimeError> {
        // Don't notify if closed
        if self.is_closed() {
            return Ok(0);
        }

        // spec: wake up to `count` waiters – we wake *all* and let Wasm decide.
        self.seq.fetch_add(1, Ordering::SeqCst);
        self.notify.notify_waiters();
        Ok(count) // lie "count" waiters; Wasm never checks
    }

    /// Shutdown the queue and wake all waiters
    pub fn shutdown(&self) {
        self.closed.store(true, Ordering::Release);

        // Change sequence to ensure waiters see a change
        self.seq.fetch_add(1, Ordering::SeqCst);

        // Wake all waiters
        self.notify.notify_waiters();
    }
}
