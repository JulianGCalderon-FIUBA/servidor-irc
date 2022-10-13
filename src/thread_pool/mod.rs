use crate::thread_pool::worker::Worker;
use std::sync::{mpsc, Arc, Mutex};

use crate::server::MAX_CLIENTS;

/// This module contains a worker's functionality. A worker manages a single thread.
pub mod worker;

/// Represents a ThreadPool.
pub struct ThreadPool {
    /// a ThreadPool has a vector of workers, to store each active thread.
    workers: Vec<Worker>,
    /// a ThreadPool sends a Job to a Worker via a channel.
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new ThreadPool with n workers.
    ///  
    pub fn create() -> Self {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(MAX_CLIENTS);

        for id in 0..MAX_CLIENTS {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
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
                eprintln!("Error: {:?}", error);
            }
        }
    }
}

impl Drop for ThreadPool {
    /// Makes sure to join each thread in the ThreadPool and drops sender.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                if let Err(error) = thread.join() {
                    eprintln!("Error: {:?}", error);
                }
            }
        }
    }
}
