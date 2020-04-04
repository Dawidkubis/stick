use std::thread;

pub struct ThreadPool(Vec<thread::JoinHandle<()>>);

impl ThreadPool {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn spawn(&mut self, fun: impl std::marker::Send + Fn() -> ()) {
		self.0.push(thread::spawn(fun));
	}
}
