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

//hotel add food group
pub struct HF01;

impl DataApi for HF01 {

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
        let table = db.get_table("food_group").expect("food_group table not exist.");
        let name = json_str!(msg; "body", "name");
        let customer_id = json_i64!(msg; "head", "userId");
        let mut desk_group = json!("{}");
        json_set!(&mut desk_group; "name"; name);
        let now = time::get_time();
        json_set!(&mut desk_group; "create_time"; now.sec);
        json_set!(&mut desk_group; "customer_id"; customer_id);

        table.save(&desk_group, &json!("{}"));

        Result::Ok(json!("{}"))
    }
}

//get group list
pub struct HF02;

impl DataApi for HF02 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32>
    {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32>
    {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32>
    {
        let customer_id = json_i64!(msg; "head", "userId");
        let table = db.get_table("food_group").expect("st table not exists.");

        let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);
        json_set!(&mut cond_json; "customer_id"; customer_id);

        let sort = json_str!(msg; "body", "sort");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let op = format!(r#"
            {{
                "offset":{},
                "limit":{}
            }}
        "#, offset, limit);
        let mut json_op = json!(&op);
        json_set!(&mut json_op; "sort"; json!(sort));
        let c_data = try!(table.count(&cond_json, &json!("{}")));
        let mut data = try!(table.find(&cond_json, &json!("{}"), &json_op));
        {
            let count = json_i64!(&c_data; "data", "0", "count");
            json_set!(&mut data;"count";count);
        }
        Result::Ok(data)
    }
}

//delete group
pub struct HF03;

impl DataApi for HF03 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32>
    {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32>
    {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32>
    {
        let table = db.get_table("food_group").expect("st table not exists.");
        let mut cond = json!("{}");
        let group_id = json_i64!(msg; "body", "id");
        let customer_id = json_i64!(msg; "head", "userId");
        json_set!(&mut cond; "id"; group_id);
        json_set!(&mut cond; "customer_id"; customer_id);
        let op = json!("{}");
        let _ = table.remove(&cond, &op);

        Result::Ok(json!("{}"))
    }

}

//hotel add food
pub struct HF04;

impl DataApi for HF04 {

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
        let table = db.get_table("food").expect("food table not exist.");
        let name = json_str!(msg; "body", "name");
        let des = json_str!(msg; "body", "des");
        let group_id = json_i64!(msg; "body", "group_id");
        let price = json_i64!(msg; "body", "price");
        let file_id = json_i64!(msg; "body", "file_id");
        let customer_id = json_i64!(msg; "head", "userId");

        let mut set = json!("{}");
        json_set!(&mut set; "name"; name);
        json_set!(&mut set; "des"; des);
        let now = time::get_time();
        json_set!(&mut set; "create_time"; now.sec);
        json_set!(&mut set; "customer_id"; customer_id);
        json_set!(&mut set; "group_id"; group_id);
        json_set!(&mut set; "file_id"; file_id);
        json_set!(&mut set; "price"; price);

        table.save(&set, &json!("{}"));

        Result::Ok(json!("{}"))
    }
}

//get food list
pub struct HF05;

impl DataApi for HF05 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32>
    {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32>
    {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32>
    {
        let customer_id = json_i64!(msg; "head", "userId");
        let table = db.get_table("food").expect("food table not exists.");

        let cond = json_str!(msg; "body", "cond");
        let mut cond_json = json!(cond);
        json_set!(&mut cond_json; "customer_id"; customer_id);

        let sort = json_str!(msg; "body", "sort");
        let limit = json_i64!(msg; "body", "limit");
        let offset = json_i64!(msg; "body", "offset");
        let op = format!(r#"
            {{
                "offset":{},
                "limit":{}
            }}
        "#, offset, limit);
        let mut json_op = json!(&op);
        json_set!(&mut json_op; "sort"; json!(sort));
        let c_data = try!(table.count(&cond_json, &json!("{}")));
        let mut data = try!(table.find(&cond_json, &json!("{}"), &json_op));
        {
            let count = json_i64!(&c_data; "data", "0", "count");
            json_set!(&mut data;"count";count);
        }
        Result::Ok(data)
    }
}