extern crate dc;
use dc::{MyDbPool, DataBase};

use std::sync::Arc;

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/order_sys";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 5);
    let my_db = DataBase::new("main", Arc::new(my_pool));

    let rst = my_db.execute("select * from forder");
    let _ = rst.and_then(|json| {
        println!("{}", json);
        Result::Ok(())
    });
}
