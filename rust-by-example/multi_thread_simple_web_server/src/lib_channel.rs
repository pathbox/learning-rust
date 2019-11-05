use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
// --snip--

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
struct Job;
impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
        }
    }
  
    pub fn execute<F>(&self, f: F)
      where
          F: FnOnce() + Send + 'static 
    {
      let job = Box::new(f);

      self.sender.send(job).unwrap();
    }

    // --snip--
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
        let thread = thread::spawn(move || {
          // loop {
          //   let job = receiver.lock().unwrap().recv().unwrap();

          //   println!("Worker {} got a job; executing.", id);
            
          //   job.call_box();
          // }
           while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });


        Worker {
            id,
            thread,
        }
    }
}