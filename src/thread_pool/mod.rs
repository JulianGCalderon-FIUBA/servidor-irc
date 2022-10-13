use crate::thread_pool::worker::Worker;
use std::sync::{mpsc, Arc, Mutex};

use crate::server::MAX_CLIENTS;

pub mod worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
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
