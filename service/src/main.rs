extern crate dc;
use dc::{MyDbPool, DataBase};

use std::sync::Arc;

extern crate service;

#[macro_use]
extern crate easy_util;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

fn main() {
    let dsn = "postgresql://postgres:1988lm@localhost/order_sys";
    let my_pool:MyDbPool = MyDbPool::new(dsn, 1);
    let my_db = DataBase::new("main", Arc::new(my_pool));
    let table = my_db.get_table("customer").unwrap();
    let mut cus = json!("{}");
    json_set!(&mut cus; "username"; "test");
    json_set!(&mut cus; "nickname"; "test");
    json_set!(&mut cus; "password"; "123456");
    println!("{}", cus);
    let _ = table.save(&cus, &json!("{}"));

    json_set!(&mut cus; "username"; "test2");
    let _ = table.save(&cus, &json!("{}"));
}
