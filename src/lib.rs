use std::net::TcpListener;

mod errors;

use errors::StickError;

/// representation of a http server
struct Stick {
	listener: TcpListener,
}

impl Stick {
	pub fn ignite(bind: &str) -> Result<Self, StickError> {
		let listener = TcpListener::bind(bind)?;

		Ok(Self {
			listener,
		})
	}
}
