pub trait Guard {
	fn guard(self) -> Self;
}

pub struct Route {
	pub uri: String,
	pub guards: Vec<Box<dyn Guard>>,
}
