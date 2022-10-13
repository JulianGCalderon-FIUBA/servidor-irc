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
            // si usamos otra estructura (while let, if let, match el job no se dropea hasta que termina, haciendo que no pueda haber muchos threads a la vez.)
            let job = receiver
                .lock()
                .expect("Lock is poisoned")
                .recv()
                .expect("Sender (threadPool) has disconnected");
            println!("Worker {id} got a job; executing");
            job();
        });

        Self {
            _id: id,
            thread: Some(thread),
        }
    }
}
