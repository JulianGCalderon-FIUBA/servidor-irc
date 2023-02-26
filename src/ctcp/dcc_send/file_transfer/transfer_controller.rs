use std::sync::{atomic::AtomicBool, Arc};

#[derive(Clone)]
pub struct TransferController {
    cancelled: Arc<AtomicBool>,
}

impl TransferController {
    pub fn new(cancelled: Arc<AtomicBool>) -> Self {
        Self { cancelled }
    }

    pub fn cancel(&mut self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}
