#![allow(dead_code)]
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

#[derive(Debug)]
/// Errors that can occur during thread pool creation
pub enum PoolCreationError {
    /// Zero-sized thread pools are not valid
    EmptyPool,
    // Homework: what other errors could be added here?
}

/// A pool of threads that can asynchronously execute jobs
///
/// Once a `ThreadPool` is created, it will generate a given number of threads,
/// which can asynchronously execute arbitrary jobs passed via [`execute`].
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a pool of threads with the specified size.
    ///
    /// Note: A pool size of zero will result in an error
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::EmptyPool)
        }

        #[allow(unused_mut)]
        let mut workers = Vec::with_capacity(size);

        // Create a channel that will live as long as the `ThreadPool` lives
        // `ThreadPool` will own the `Sender` and use it to send jobs to the
        // next available worker
        let (sender, receiver) = mpsc::channel();

        // We need to pass the receiver to each of the workers. But we cannot
        // have multiple receivers in a Multi-producer, Single-consumer channel!
        // The solution is to wrap the receiver in Arc and Mutex so that we can
        // safely share multiple references to the receiver across workers.
        // TODO: Create reference counted mutex for receiver

        for id in 0..size {
            // TODO: Uncomment me!
            // workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(ThreadPool {
            workers,
            sender,
        })
    }

    /// Sends the job to a worker that will execute the job when available
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        // We cannot send a value of unknown size unless we use `Box`
        // Note: `Job` is a type alias for `Box<dyn FnOnce...>`
        //       `Box<dyn T>` is a trait object which uses dynamic dispatch to
        //       invoke our closure. The `Box` is a pointer that references
        //       the code block we want to execute
        let job: Job = Box::new(f);

        // TODO: Send job to channel
        // Hint: Who owns the sender?
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job: Job;
                // TODO: Read jobs from the channel and store in variable
                // TODO: Uncomment me!
                // job.execute();
            }
        });

        Worker {
            id,
            thread,
        }
    }
}

// Hack for the compiler to properly take ownership of the value inside the Box
// For more info on why this is needed, read the following:
// https://doc.rust-lang.org/book/ch20-02-multithreaded.html#implementing-the-execute-method
trait FnBox {
    fn execute(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn execute(self: Box<F>) {
        (*self)()
    }
}
