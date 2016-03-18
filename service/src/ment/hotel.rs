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

//hotel register 
pub struct H01;

impl DataApi for H01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        Result::Ok(DigestUtil::empty_key()) 
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        println!("{}", msg);
        let table = db.get_table("customer").expect("customer table not exist.");
        let username = json_str!(msg; "body", "username");
        let password = json_str!(msg; "body", "password");
        let mut customer = json!("{}");
        json_set!(&mut customer; "username"; username);
        json_set!(&mut customer; "password"; password);
        let now = time::get_time();
        json_set!(&mut customer; "reg_time"; now.sec);
        json_set!(&mut customer; "type"; CONS.code_to_id("user_type", "hotel").unwrap());
        let mut op = json!(r#"
            {
                "ret": {
                    "id":1
                }
            }
        "#);
        let rst = try!(table.save(&customer, &op));
        let customer_id = json_i64!(&rst; "data", "0", "id");

        let owner = json_str!(msg; "body", "owner");
        let owner_phone = json_str!(msg; "body", "owner_phone");
        let hotel_name = json_str!(msg; "body", "hotel_name");
        let hotel_addr = json_str!(msg; "body", "hotel_addr");
        let h_table = db.get_table("hotel").expect("hotel table not exist.");
        let mut hotel = json!("{}");
        json_set!(&mut hotel; "id"; customer_id);
        json_set!(&mut hotel; "owner"; owner);
        json_set!(&mut hotel; "owner_phone"; owner_phone);
        json_set!(&mut hotel; "name"; hotel_name);
        json_set!(&mut hotel; "addr"; hotel_addr);
        let _ = h_table.save(&hotel, &op);
        Result::Ok(json!("{}"))
    }
}

//hotel query info 
pub struct H02;

impl DataApi for H02 {

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
        let customer_id = json_i64!(msg; "head", "userId");

        let table = db.get_table("customer").expect("customer table not exist.");
        let mut cond = json!("{}"); 
        json_set!(&mut cond; "id"; customer_id);
        let data = json!("{}");
        let op = json!("{}");
        let customer = try!(table.find_one(&cond, &data, &op));

        let hotel_table = db.get_table("hotel").expect("hotel table not exist.");
        let hotel = try!(hotel_table.find_one(&cond, &data, &op));

        let mut back_json = json!("{}");
        json_set!(&mut back_json; "customer"; customer);
        json_set!(&mut back_json; "hotel"; hotel);

        Result::Ok(back_json)
    }
}







