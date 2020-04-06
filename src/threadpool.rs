use std::marker::Send;
use std::sync::mpsc;
use std::thread;

struct Worker(thread::JoinHandle<()>);

impl Worker {
	fn new(receiver: mpsc::Receiver) -> Self {
		Self(thread::spawn(|| {}))
	}
}

pub struct ThreadPool(Vec<Worker>);

impl ThreadPool {
	pub fn new(capacity: usize) -> Result<Self, StickError> {

		if capacity < 0 {
			return Err(StickError::ThreadPoolError);
		}

		let (sender, receiver) = mpsc::channel();

		let workers: Vec<Worker> = (0..capacity).iter()
			.map(Worker::new())
			.collect();

		Ok(Self(workers))
	}

	pub fn spawn<T>(&mut self, fun: T)
	where
		T: Send + Fn() -> () + 'static,
	{
		self.0.push(thread::spawn(fun));
	}
}
