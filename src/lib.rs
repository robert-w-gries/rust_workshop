#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver};
use std::thread;

#[derive(Debug)]
pub enum PoolCreationError {
    EmptyPool,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::EmptyPool)
        }

        let mut workers = Vec::with_capacity(size);

        // TODO: Create reference counted mutex for receiver
        let (sender, receiver) = mpsc::channel();

        for id in 0..size {
            // workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(ThreadPool {
            workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        // TODO: Send job to channel
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // TODO: Read jobs from channel and execute them
        });

        Worker {
            id,
            thread,
        }
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
