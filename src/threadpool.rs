use std::marker::Send;
use std::thread;

pub struct ThreadPool(Vec<thread::JoinHandle<()>>);

impl ThreadPool {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn spawn<T>(&mut self, fun: T)
	where
		T: Send + Fn() -> () + 'static,
	{
		self.0.push(thread::spawn(fun));
	}
}
