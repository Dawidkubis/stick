#[test]
fn hello_world() {
	use std::thread;
	use stick::Strick;

	#[get("/")]
	fn index() -> String {
		"hello world".owned()
	}

	thread::spawn(|| {
		Stick::ignite("/").mount(index).launch();
	});
}
