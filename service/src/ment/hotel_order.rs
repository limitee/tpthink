use super::super::util::DigestUtil;
use super::super::dc::DataBase;
use super::super::dc::MyDbPool;
use super::super::cons::CONS;
use super::super::cons::ErrCode;

use std::collections::BTreeMap;
use std::io::Read;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate regex;
use self::regex::Regex;

extern crate time;

use super::super::inter::{DataApi};
use super::super::sv_util::{KeyHelper};

//hotel add order
pub struct HO01;

impl DataApi for HO01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        println!("{}", msg);
        let forder_table = db.get_table("forder").expect("forder table not exist.");
        let desk_table = db.get_table("desk").expect("forder table not exist.");

        let desk_id = json_i64!(msg; "body", "desk_id");
        let customer_id = json_i64!(msg; "head", "userId");

        let mut order = json!("{}");
        json_set!(&mut order; "price"; 0);
        json_set!(&mut order; "customer_id"; customer_id);
        let order_status = CONS.code_to_id("order_status", "eat").unwrap();
        json_set!(&mut order; "status"; order_status);
        let now = time::get_time();
        json_set!(&mut order; "create_time"; now.sec);

        let mut op = json!("{}");
        let mut ret = json!("{}");
        json_set!(&mut ret; "id"; 1);
        json_set!(&mut op; "ret"; ret);
        let order_rst = try!(forder_table.save(&order, &op));

        let order_id = json_i64!(&order_rst; "data", "0", "id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; desk_id);

        let mut doc = json!("{}");
        let mut set = json!("{}");
        json_set!(&mut set; "status"; CONS.code_to_id("desk_status", "eat").unwrap());
        json_set!(&mut set; "cur_order_id"; order_id);
        json_set!(&mut doc; "$set"; set);

        let op = json!("{}");
        let _ = desk_table.update(&cond, &doc, &op);

        let mut back_json = json!("{}");
        json_set!(&mut back_json; "order_id"; order_id);
        Result::Ok(back_json)
    }
}

//hotel add order food
pub struct HO02;

impl DataApi for HO02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        //println!("{}", msg);
        let table = db.get_table("order_food").expect("order_food table not exist.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");
        let order_id = json_i64!(body; "order_id");
        let data = json_path!(body; "data");
        for set in data.as_array().unwrap() {
            let food_id = json_i64!(set; "food_id");
            let num = json_i64!(set; "num");
            let amount = json_i64!(set; "amount");
            let price = json_i64!(set; "price");
            let name = json_str!(set; "name");

            let mut conflict = json!("{}");
            json_set!(&mut conflict; "customer_id"; 1);
            json_set!(&mut conflict; "forder_id"; 1);
            json_set!(&mut conflict; "food_id"; 1);

            let mut up_data = json!("{}");

            let mut inc_data = json!("{}");
            json_set!(&mut inc_data; "num"; num);
            json_set!(&mut inc_data; "amount"; amount);
            json_set!(&mut up_data; "$inc"; inc_data);

            let mut data = json!("{}");
            json_set!(&mut data; "num"; num);
            json_set!(&mut data; "amount"; amount);
            json_set!(&mut data; "customer_id"; customer_id);
            json_set!(&mut data; "forder_id"; order_id);
            json_set!(&mut data; "food_id"; food_id);
            json_set!(&mut data; "name"; name);
            json_set!(&mut data; "price"; price);

            let op = json!("{}");
            let _ = table.upsert(&conflict, &data, &up_data, &op);
        }
        Result::Ok(json!("{}"))
    }
}

//hotel get order food
pub struct HO03;

impl DataApi for HO03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        println!("{}", msg);
        let table = db.get_table("order_food").expect("order_food table not exist.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");
        let order_id = json_i64!(body; "order_id");

        let mut cond = json!("{}");
        json_set!(&mut cond; "forder_id"; order_id);
        json_set!(&mut cond; "customer_id"; customer_id);

        let data = json!("{}");
        let op = json!("{}");

        let rst = table.find(&cond, &data, &op).unwrap();

        let mut back_json = json!("{}");
        json_set!(&mut back_json; "order_food"; rst);

        //get the order info
        let table = db.get_table("forder").expect("forder table not exist.");

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; order_id);

        let data = json!("{}");
        let op = json!("{}");

        let rst = table.find(&cond, &data, &op).unwrap();
        json_set!(&mut back_json; "order"; rst);

        Result::Ok(back_json)
    }
}

//finish the order
pub struct HO04;

impl DataApi for HO04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        info!("{}", msg);
        let table = db.get_table("forder").expect("forder table not exist.");
        let customer_id = json_i64!(msg; "head", "userId");
        let body = json_path!(msg; "body");
        let order_id = json_i64!(body; "order_id");

        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; order_id);
        json_set!(&mut cond; "customer_id"; customer_id);

        let mut doc = json!("{}");
        let mut set_data = json!("{}");
        json_set!(&mut set_data; "status"; CONS.code_to_id("order_status", "finish").unwrap());
        json_set!(&mut doc; "$set"; set_data);

        let rst = table.update(&cond, &doc, &json!("{}"));
        let rst = rst.and_then(|_| {
            let table = db.get_table("desk").expect("desk table not exist.");

            let mut cond = json!("{}");
            json_set!(&mut cond; "cur_order_id"; order_id);

            let mut doc = json!("{}");
            let mut set_data = json!("{}");
            json_set!(&mut set_data; "status"; CONS.code_to_id("desk_status", "idle").unwrap());
            json_set!(&mut set_data; "cur_order_id"; -1);
            json_set!(&mut doc; "$set"; set_data);

            table.update(&cond, &doc, &json!("{}"))
        });

        let rst = rst.and_then(|_| {
            Result::Ok(json!("{}"))
        });

        rst
    }
}