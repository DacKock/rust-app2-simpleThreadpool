use std::sync::{Arc, /* Condvar, */ Mutex};
use std::thread::JoinHandle;

type Task = Box<dyn FnOnce() + Send>;

// You can define new types (e.g., structs) if you need.
// However, they shall not be public (i.e., do not use the `pub` keyword).

use std::{sync::mpsc, thread};

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Task>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            // let (lock, cond) = &*receiver;
            // let mut guard = lock.lock().unwrap().recv();
            
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                    // cond.notify_one();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// The thread pool.
pub struct Threadpool {
    // Add here any fields you need.
    // We suggest storing handles of the worker threads, submitted tasks,
    // and an information whether the pool is running or it is to be finished.

    workers: Vec<Worker>,
    sender: Option<mpsc::SyncSender<Task>>,
}

impl Threadpool {
    /// Create new thread pool with `workers_count` workers.
    pub fn new(workers_count: usize) -> Self {
        //unimplemented!("Initialize necessary data structures.");
        assert!(workers_count > 0);

        let (sender, receiver) = mpsc::sync_channel(0);
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(workers_count);

        for id in 0..workers_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Threadpool {
            workers,
            sender: Some(sender),
        }
    }

    /// Submit a new task.
    pub fn submit(&self, task: Task) {
        //unimplemented!("We suggest saving the task, and notifying the worker(s)");
        let job = Box::new(task);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }

    // We suggest extracting the implementation of the worker to an associated
    // function, like this one (however, it is not a part of the public
    // interface, so you can delete it if you implement it differently):
    // fn worker_loop(/* unimplemented!() */) {
    //     unimplemented!("Initialize necessary variables.");

    //     loop {
    //         unimplemented!("Wait for a task and then execute it.");
    //         unimplemented!(
    //             "If there are no tasks, and the thread pool is to be finished, break the loop."
    //         );
    //         unimplemented!("Be careful with locking! The tasks shall be executed concurrently.");
    //     }
    // }
}

impl Drop for Threadpool {
    /// Gracefully end the thread pool.
    ///
    /// It waits until all submitted tasks are executed,
    /// and until all threads are joined.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
