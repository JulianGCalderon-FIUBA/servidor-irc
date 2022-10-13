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
            let job = receiver.lock().expect("Lock is poisoned").recv().unwrap();
            println!("Worker {id} got a job; executing");
            job();
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
