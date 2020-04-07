use std::marker::Send;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::StickError;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker(thread::JoinHandle<()>);

impl Worker {
	fn new(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
		let handle = thread::spawn(move || loop {
			let job = receiver.lock().unwrap().recv().expect("recverror");

			job();
		});

		Self(handle)
	}
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>,
}

impl ThreadPool {
	pub fn new(capacity: usize) -> Result<Self, StickError> {
		if capacity < 0 {
			return Err(StickError::ThreadPoolError);
		}

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let workers: Vec<Worker> = (0..capacity)
			.map(|x| Worker::new(Arc::clone(&receiver)))
			.collect();

		Ok(Self { workers, sender })
	}

	pub fn execute<T>(&mut self, fun: T)
	where
		T: Send + FnOnce() + 'static,
	{
		let job = Box::new(fun);

		self.sender.send(job).unwrap();
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn hello() {
		use crate::threadpool::ThreadPool;
		use std::thread;

		let mut threadpool = ThreadPool::new(4).unwrap();
		for i in 0..10 {
			threadpool.execute(|| {
				let t = thread::current();
				println!("hello from thread: {:?}", t.id())
			});
		}
	}
}
