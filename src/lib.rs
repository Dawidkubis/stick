mod errors;
use errors::StickError;

use std::{
	net::TcpListener,
	io::Read,
	};

/// Represents a http server
pub struct Server {
	listener: TcpListener,
}

impl Server {
	/// Creates a new `Server` instance
	// TODO https
	pub fn new(bind: &str) -> Result<Self, StickError> {
		let listener = TcpListener::bind(bind)?;

		Ok(Self {
			listener,
		})
	}
}

impl Iterator for Server {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		let (mut stream, _address) = self.listener.accept()
			.ok()?;
		
		let mut buffer = [0; 1024]; // TODO buffer size
		
		stream.read(&mut buffer).ok()?;

		let buffer = String::from_utf8_lossy(&buffer).to_string();
		Some(buffer)
	}
}
