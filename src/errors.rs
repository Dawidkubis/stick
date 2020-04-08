use thiserror::Error;

#[derive(Error, Debug)]
pub enum StickError {
	#[error("unable to bind port: {0}")]
	PortError(#[from] std::io::Error),
	#[error("unable to send job")]
	SenderError,
}
