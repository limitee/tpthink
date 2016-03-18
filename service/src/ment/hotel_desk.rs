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

//hotel add desk group
pub struct HD01;

impl DataApi for HD01 {

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
        let table = db.get_table("desk_group").expect("desk_group table not exist.");
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
pub struct HD02;

impl DataApi for HD02 {

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
        let table = db.get_table("desk_group").expect("st table not exists.");

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
pub struct HD03;

impl DataApi for HD03 {

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
        let table = db.get_table("desk_group").expect("st table not exists.");
        let mut cond = json!("{}");
        let group_id = json_i64!(msg; "body", "id");
        let customer_id = json_i64!(msg; "head", "userId");
        json_set!(&mut cond; "id"; group_id);
        json_set!(&mut cond; "customer_id"; customer_id);
        let op = json!("{}");
        table.remove(&cond, &op);

        Result::Ok(json!("{}"))
    }

}

//hotel add desk
pub struct HD04;

impl DataApi for HD04 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("desk").expect("desk table not exist.");
        let group_id = json_i64!(msg; "body", "group_id");
        let name = json_str!(msg; "body", "name");
        let customer_id = json_i64!(msg; "head", "userId");

        let mut desk = json!("{}");
        let now = time::get_time();
        json_set!(&mut desk; "name"; name);
        json_set!(&mut desk; "group_id"; group_id);
        json_set!(&mut desk; "create_time"; now.sec);
        json_set!(&mut desk; "customer_id"; customer_id);

        table.save(&desk, &json!("{}"));

        Result::Ok(json!("{}"))
    }
}

//get desk list
pub struct HD05;

impl DataApi for HD05 {

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
        let table = db.get_table("desk").expect("st table not exists.");

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

//delete the desk
pub struct HD06;

impl DataApi for HD06 {

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
        let table = db.get_table("desk").expect("st table not exists.");
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