use std::marker::Send;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::StickError;

type J = Box<dyn FnOnce() + Send + 'static>;

enum Msg {
	Terminate,
	Job(J)
}

struct Worker(Option<thread::JoinHandle<()>>);

impl Worker {
	fn new(receiver: Arc<Mutex<mpsc::Receiver<Msg>>>) -> Self {
		let handle = thread::spawn(move || loop {
			let job = loop {
				match receiver.lock().unwrap().recv() {
					Ok(s) => break s,
					Err(e) => (),
				};
			};

			let job = match job {
				Msg::Job(j) => j,
				Msg::Terminate => break,
			};

			job();
		});

		Self(Some(handle))
	}
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Msg>,
}

impl ThreadPool {
	pub fn new(capacity: usize) -> Self {
		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let workers: Vec<Worker> = (0..capacity)
			.map(|x| Worker::new(Arc::clone(&receiver)))
			.collect();

		Self {
			workers,
			sender,
		}
	}

	pub fn execute<T>(&mut self, fun: T)
	where
		T: Send + FnOnce() + 'static,
	{
		let job = Box::new(fun);

		self.sender.send(Msg::Job(job)).unwrap();
	}
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
    	for x in self.workers.iter_mut() {
    		self.sender.send(Msg::Terminate).unwrap();
    		if let Some(handle) = x.0.take(){ handle.join().unwrap() };
 		}			
    }
}

#[test]
fn hello() {
	use crate::threadpool::ThreadPool;
	use std::thread;

	let (sender, receiver) = mpsc::channel();
	let mut stuff: Vec<usize> = vec![];

	let mut threadpool = ThreadPool::new(4);
	for i in 0..10 {
		let s = sender.clone();
		threadpool.execute(move || {
			s.send(1).unwrap();
		});
		stuff.push(receiver.recv().unwrap())
	}

	assert_eq!(stuff.iter().sum::<usize>(), 10);
}
