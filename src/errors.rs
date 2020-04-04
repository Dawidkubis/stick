use thiserror::Error;

#[derive(Error, Debug)]
pub enum StickError {
	#[error("unable to bind port")]
	PortError(#[from] std::io::Error),
}
