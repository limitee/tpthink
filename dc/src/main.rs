extern crate dc;
use dc::{MyDbPool, DataBase};

use std::sync::Arc;

#[macro_use]
extern crate log;
extern crate elog;

fn main() {
	let _ = elog::init();
    let dsn = "postgresql://postgres:1988lm@localhost/mcp";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 5);
    let my_db = DataBase::new("main", Arc::new(my_pool));
	
    let rst = my_db.execute("select * from forder");
    let _ = rst.and_then(|json| {
        println!("{}", json);
        Result::Ok(())
    });
    
    let _ = my_db.stream("select * from forder", |json| {
        println!("{}", json);
        true
    });
}
