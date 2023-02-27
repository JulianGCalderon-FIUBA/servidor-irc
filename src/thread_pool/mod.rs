use crate::thread_pool::worker::Worker;
use std::sync::{mpsc, Arc, Mutex};

/// This module contains a worker's functionality. A worker handles a single thread.
pub mod worker;

/// Represents a ThreadPool.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new [`ThreadPool`] with n workers.
    ///  
    pub fn create(pool_size: usize) -> Self {
        assert!(pool_size != 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(pool_size);

        for _ in 0..pool_size {
            workers.push(Worker::new(Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// Sends Job received to Worker.
    ///
    /// # Panics
    ///
    /// `execute` fails if the Job could not be sent.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(sender) = self.sender.as_ref() {
            if let Err(error) = sender.send(job) {
                eprintln!("Error: {error:?}");
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if let Err(error) = thread.join() {
                    eprintln!("Error: {error:?}");
                }
            }
        }
    }
}
