use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::thread_pool::Job;

/// A Worker represents an active thread.
pub struct Worker {
    _id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Creates new Worker that executes Job received through receiver.
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().expect("Lock is poisoned").recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    break;
                }
            }
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
