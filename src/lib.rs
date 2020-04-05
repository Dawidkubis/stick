use std::net::*;

mod errors;
mod threadpool;

use errors::StickError;
use threadpool::ThreadPool;

struct Stick {
	listener: TcpListener,
	thread_pool: ThreadPool,
}

impl Stick {
	pub fn ignite(port: u16) -> Result<Self, StickError> {
		let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

		Ok(Self {
			listener,
			thread_pool: ThreadPool::new(),
		})
	}

	pub fn throw(self) {
		loop {
			
		}
	}
}
