//! Synchronization items

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/////////////////////////
// Cancellation tokens //
/////////////////////////
// Based on https://stackoverflow.com/a/78811321
// Ordering is a giant mystery imo
// The whole "no read if ordered before" feels inverted

#[derive(Debug, Clone)]
/// Analogous to C&sharp;'s CancellationToken.
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    #[inline]
    /// Returns whether the token has been used.
    pub fn cancelled(&self) -> bool { self.cancelled.load(Ordering::Acquire) }
}

#[derive(Debug, Clone)]
/// Owner of a cancellation token.
pub struct CancellationSource {
    cancelled: Arc<AtomicBool>,
}

impl CancellationSource {
    #[inline]
    /// Uses to the token, cancelling any operations.
    pub fn cancel(&self) { self.cancelled.store(true, Ordering::Release); }
}

/// Creates a cancellation token pair, one to cancel, one to check cancellation.
pub fn cancellation_token() -> (CancellationSource, CancellationToken) {
    let signal = Arc::new(AtomicBool::new(false));
    let source = CancellationSource { cancelled: signal.clone() };
    let token = CancellationToken { cancelled: signal };
    (source, token)
}
