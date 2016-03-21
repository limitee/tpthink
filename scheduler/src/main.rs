extern crate scheduler;
use scheduler::*;

use std::io::prelude::*;
use std::net::TcpStream;

use std::thread;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").unwrap();

    // ignore the Result
    let _ = stream.write(&[1]);
    //let _ = stream.read(&mut [0; 128]); // ignore here too
	loop {
		thread::sleep(std::time::Duration::new(1, 0));
		let _ = stream.write(&[1]);
	}
	// the stream is closed here
}
