use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::thread_pool::Job;

pub struct Worker {
    _id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            match receiver.lock() {
                Ok(receiver) => {
                    if let Ok(job) = receiver.recv() {
                        println!("Worker {id} started");
                        job();
                    }
                }
                Err(error) => eprintln!("Error: {:?}", error),
            };
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
