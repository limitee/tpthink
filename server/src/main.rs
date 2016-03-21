use std::thread;
use std::net::{TcpListener, TcpStream};

fn main() {
	
	let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
	
	fn handle_client(stream: TcpStream) {
	    // ...
	}
	
	// accept connections and process them, spawning a new thread for each one
	for stream in listener.incoming() {
	    match stream {
	        Ok(stream) => {
	            thread::spawn(move|| {
	                // connection succeeded
	                handle_client(stream)
	            });
	        }
	        Err(e) => { /* connection failed */ }
	    }
	}
	
	// close the socket server
	drop(listener);
}
