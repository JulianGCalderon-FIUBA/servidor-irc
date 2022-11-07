use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::thread_pool::Job;

/// A Worker represents an active thread.
pub struct Worker {
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates new [`Worker`] that executes Job received through receiver.
    pub fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().expect("Lock is poisoned").recv();
            match message {
                Ok(job) => {
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Self {
            thread: Some(thread),
        }
    }
}
