use std::sync::{atomic::AtomicBool, Arc};

/// Should not be created manually, its created with a FileTransfer
/// Controls the status of FileTransfer
#[derive(Clone)]
pub struct TransferController {
    cancelled: Arc<AtomicBool>,
}

impl TransferController {
    pub fn new(cancelled: Arc<AtomicBool>) -> Self {
        Self { cancelled }
    }

    /// Cancels the corresponding FileTransfer
    pub fn cancel(&mut self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
