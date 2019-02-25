extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::AsyncRead;
use tokio_io::io::copy;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
	// Create the event loop that will drive this server
    let mut core = Core::new().unwrap();
	let handle = core.handle();

	//Bind the server's socket
	let addr = "127.0.0.1:12345".parse().unwrap();
	let listener = TcpListener::bind(&addr, &handle).unwrap();

	//Pull out a stream of sockets for incoming connections
	let server = listener.incoming().for_each(|(sock, _)|{
		//Split up the reading and writing parts of the socket
		let (reader, writer) = sock.split();

		//A future that echoes the data and returns how many
		//bytes were copied...
		let bytes_copied = copy(reader, writer);

		// ... after which we'll print what happened
		let handle_conn = bytes_copied.map(|amt|{
			println!("write {:?} bytes", amt)
		}).map_err(|err|{
			println!("IO error {:?}", err)
		});

		handle.spawn(handle_conn);
		Ok(())
	});

	core.run(server).unwrap();
}
