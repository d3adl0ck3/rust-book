use std::{sync::{mpsc, Arc, Mutex}, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker {
        // Using move ends up copying the id to the thread so we can print it
        let thread = thread::spawn(move|| loop {
            println!("Worker {id} is ready");
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} is executing job");
                    job();

                },
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }

            }
        });
        Worker{id,thread:Some(thread)}
    }
}
#[derive(Debug)]
pub struct PoolCreationError;
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        Self::build(size).expect("Cannot create 0 sized threadpool")
    }
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>{
        if size == 0 {
            Err(PoolCreationError{})
        } else {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool{workers, sender: Some(sender)})
        }
    }
    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);
            self.sender.as_ref().unwrap().send(job).unwrap();
        }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        println!("Dropping ThreadPool");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
