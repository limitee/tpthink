use std::thread;
use std::net::{TcpListener, TcpStream};

use std::io::Read;

fn main() {
	
	let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
	
	fn handle_client(mut stream: TcpStream) {
		let mut buffer = Vec::<u8>::new();	//缓存大小
		let mut target_buffer_length = 0;	//当前数据块的长度
		
		let mut buf = vec![0_u8; 1024];	//收取数据的缓存大小
	    loop {
	    		let rst = stream.read(&mut buf);
	    		match rst {
	    			Ok(size) => {
	    				if size > 0 {
	    					buffer.extend_from_slice(&buf[0..size]);
	    					println!("{}", buffer.len());
	    				} else {
	    					println!("end");
	    					break;
	    				}
	    			},
	    			Err(err) => {
	    				println!("{}", err);
	    			},
	    		}
	    }
	    //connection is not available any more.
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
