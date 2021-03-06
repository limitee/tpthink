extern crate dc;
use dc::{MyDbPool, DataBase, stream};

use std::sync::Arc;

#[macro_use]
extern crate log;
extern crate elog;

#[macro_use]
extern crate easy_config;
use easy_config::CFG;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;
use std::str::FromStr;

fn main() {
	let _ = elog::init();
    info!(target:"main", "{}", CFG.get_data());
    let dsn = cfg_str!("db", "dsn");
    let my_pool:MyDbPool = MyDbPool::new(dsn, cfg_i64!("db", "conn_limit") as u32);
    let my_db = DataBase::new("main", Arc::new(my_pool));
	
    let rst = my_db.execute("select * from customer");
    let _ = rst.and_then(|json| {
        println!("{}", json);
        Result::Ok(())
    });
    
    let conn = my_db.get_connection().unwrap();
    let _ = stream(conn, "select * from customer", move |json| {
        //let rst = my_db.execute("select * from customer");
        println!("{}", json);
        true
    });
}
