use std::net::*;

mod errors;
mod threadpool;
mod route;

use errors::StickError;
use threadpool::ThreadPool;

struct Stick {
	listener: TcpListener,
	routes: Vec<()>,
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

	pub fn mount(&mut self) {

	}

	pub fn throw(self) {
		self.listener.incoming()
			.map(self.handle_connection)
	}

	fn handle_connection(&self, stream: TcpStream) {
			
	}
}
