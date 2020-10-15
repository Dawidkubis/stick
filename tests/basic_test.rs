#[test]
fn hello_world() {
	use stick::Server;

	let stick = Server::new("127.0.0.1:8000").unwrap();

	for i in stick {
		println!("{}", i)
	}
}
