use std::thread;
use std::net::{TcpListener, TcpStream};

use std::io::Read;
use std::io::Cursor;

#[macro_use]
extern crate log;
extern crate elog;

extern crate protocol;
use protocol::Protocol;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use std::sync::{Arc, Mutex};
use std::collections::BTreeMap;

type Msg = Json;
type ManMsg = Json;
type IndexMap = Arc<Mutex<BTreeMap<i32, Sender<Msg>>>>;

/**
 * 服务器
 */
pub struct Server {
	url: String,
	listener: TcpListener,
	index: i32,	//计数器，为了一一对应当前登陆得用户
	index_map: IndexMap,	//map the index and the sender of connection
	
}

impl Server {
	pub fn new(url:&str) -> Server {
		let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
		Server {
			url: url.to_string(),
			listener: listener,
			index: 0_i32,
			index_map: Arc::new(Mutex::new(BTreeMap::<i32, Sender<Msg>>::new())),
		}
	}
	
	/**
	 * this method will block
	 */
	pub fn start(&mut self) {
		info!("server start at: {}", self.url);
		for stream in self.listener.incoming() {
			let index_map = self.index_map.clone();
			match stream {
		        Ok(stream) => {
			        	self.index += 1;
					let (tx, rx) = mpsc::channel::<Msg>();
		        		let rx = Arc::new(Mutex::new(rx));
		    			//clone the map
        				let index_map = index_map.lock().unwrap();
        				
		        },
		        Err(_) => {
		        	 /* connection failed */ 
		        },
		    }
		}
	}
}
