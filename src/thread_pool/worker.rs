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
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
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
