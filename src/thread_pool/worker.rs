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
            if let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} started");
                job();
            }
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
