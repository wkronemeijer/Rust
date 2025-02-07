//! Synchronization items

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/////////////////////////
// Cancellation tokens //
/////////////////////////
// Based on https://stackoverflow.com/a/78811321
// Ordering is a giant mystery imo

#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    #[inline]
    pub fn cancelled(&self) -> bool { self.cancelled.load(Ordering::Acquire) }
}

#[derive(Debug, Clone)]
pub struct CancellationSource {
    cancelled: Arc<AtomicBool>,
}

impl CancellationSource {
    #[inline]
    pub fn cancel(&self) { self.cancelled.store(true, Ordering::Release); }
}

pub fn cancellation_token() -> (CancellationSource, CancellationToken) {
    let cancelled = Arc::new(AtomicBool::new(false));
    let source = CancellationSource { cancelled: cancelled.clone() };
    let token = CancellationToken { cancelled };
    (source, token)
}
