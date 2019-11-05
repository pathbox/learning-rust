use std::thread;

use std::sync::mpsc

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id));
    }

    ThreadPool {
      workers,
      sender,
    }
  }
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Wokrer {
    let thread = thread::spawn(|| {});

    Worker {
      id, 
      thread,
    }
  }
}