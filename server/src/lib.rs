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

pub type Msg = Json;
pub type ManMsg = Json;
type IndexMap = Arc<Mutex<BTreeMap<i32, Sender<Msg>>>>;
pub type ClientRx = Arc<Mutex<Receiver<Msg>>>;

pub struct ClientHandler {
	proto: Protocol,
	man_sx: Sender<ManMsg>,
	client_rx: ClientRx,
}

impl ClientHandler {
	
	pub fn new(proto:Protocol, man_sx:Sender<ManMsg>, client_rx:ClientRx) -> ClientHandler {
		ClientHandler {
			proto: proto,
			man_sx: man_sx,
			client_rx: client_rx,
		}
	}
	
	/**
	 * 处理和client交互
	 */
	pub fn start(&mut self) {
		let rst = self.proto.rec_msg();
		let rst = rst.and_then(|(head_str, body_str)|{
			info!("head:{}", head_str);
			info!("body:{}", body_str);
			//handle_msg(&head_str, &body_str);
			//buffer = new_buffer;
			let mut body = json!("{}");
			json_set!(&mut body; "msg"; "欢迎，登陆成功");
			self.proto.send_body("S01", &body);
			Result::Ok(())
		});
		rst.and_then(|_|{
			loop {
				thread::sleep(std::time::Duration::new(1, 0));
			}
			Result::Ok(())
		});
	}
}

/**
 * 服务器
 */
pub struct Server {
	url: String,
	listener: TcpListener,
	index: i32,	//计数器，为了一一对应当前登陆得用户
	index_map: IndexMap,	//map the index and the sender of connection
	man_rx: Receiver<ManMsg>,
	man_sx: Sender<ManMsg>, 
}

impl Server {
	pub fn new(url:&str) -> Server {
		let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
		let (man_sx, man_rx) = mpsc::channel::<ManMsg>();
		Server {
			url: url.to_string(),
			listener: listener,
			index: 0_i32,
			index_map: Arc::new(Mutex::new(BTreeMap::<i32, Sender<Msg>>::new())),
			man_sx: man_sx,
			man_rx: man_rx,
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
					let index = self.index;
					
					let (sx, rx) = mpsc::channel::<Msg>();
		        		let rx = Arc::new(Mutex::new(rx));
		    			//lock the index map
        				let mut index_map = index_map.lock().unwrap();
        				index_map.insert(index, sx);
        				
        				let rx = rx.clone();
        				let man_sx = self.man_sx.clone();
        				thread::spawn(move|| {
        					let proto = Protocol::new(stream, String::from("abc"));	
        					let mut ch = ClientHandler::new(proto, man_sx, rx);
        					ch.start();
		                // connection succeeded
		                //handle_client(stream, rx, man_sx, count)
		            });
		        },
		        Err(_) => {
		        	 /* connection failed */ 
		        },
		    }
		}
	}
}
