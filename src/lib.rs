#![allow(dead_code)]
use mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
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

/// Wrapper for closure that can be sent via message passing
type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// Create a pool of threads with the specified size.
    ///
    /// Note: A pool size of zero will result in an error
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::EmptyPool);
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
        // safely share ownership of the single receiver across threads.
        // TODO #2: Create reference counted mutex for receiver
        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            // TODO #1: Uncomment me!
            workers.push(Worker::new(id, receiver.clone()));
        }

        Ok(ThreadPool { workers, sender })
    }

    /// Sends the job to a worker that will execute the job when available
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // We cannot send a value of unknown size unless we use `Box`
        // Note: `Job` is a type alias for `Box<dyn FnOnce...>`
        //       `Box<dyn T>` is a trait object which uses dynamic dispatch to
        //       invoke our closure. The `Box` is a pointer that references
        //       the code block we want to execute
        let job: Job = Box::new(f);

        // TODO #3: Send job to channel
        // Hint: Who owns the sender?
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver_mutex: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Locking the mutex provides us a smart pointer `MutexGuard<Receiver<Job>>`
                // By calling `recv()`, the smart pointer is implicitly dereferenced to grant
                // access to the `Receiver<Job>`!
                //
                // Note: `recv()` is a blocking call. This thread will be blocked until it receives
                //       a message through the channel
                let receiver = receiver_mutex.lock().unwrap();
                let job: Job = receiver.recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.execute();
            }
        });

        Worker { id, thread }
    }
}

// Hack for the compiler to properly take ownership of the closure inside the Box
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
