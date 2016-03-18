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

//user get file list
pub struct F01;

impl DataApi for F01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        let rst = KeyHelper::from_cache(db, head);
        KeyHelper::active(db, head);
        rst
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let customer_id = json_i64!(msg; "head", "userId");

        let table = db.get_table("file").expect("file table not exists.");
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

//upload file
pub struct F02;

impl DataApi for F02 {

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
        let user_id_str = json_str!(msg; "head", "userId");
        let customer_id = i64::from_str(user_id_str).unwrap();

        let req_body = json_path!(msg; "body");
        let file_type = json_str!(req_body; "type");
        let file_type_id = match CONS.code_to_id("file_type", file_type) {
            Ok(x) => x,
            Err(_) => -1,
        };
        let mut body = req_body.clone();
        json_set!(&mut body; "type"; file_type_id);
        let sec = time::get_time().sec;
        json_set!(&mut body; "create_time"; sec);
        json_set!(&mut body; "customer_id"; customer_id);
        println!("{}", body);

        let table = db.get_table("file").expect("file table not exists.");
        let op = r#"
            {
                "ret":
                {
                    "id":1
                }
            }
        "#;
        let op_json = Json::from_str(op).unwrap();
        let data = try!(table.save(&body, &op_json));

        let id = json_i64!(&data; "data", "0", "id");
        let back_body = format!(r#"
            {{
                "id":{}
            }}
        "#, id);
        Result::Ok(json!(&back_body))
    }

}

//upload file block
pub struct F03;

impl DataApi for F03 {

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
        let body = msg.find_path(&["body"]).unwrap();
        let user_id_str = msg.find_path(&["head", "userId"]).unwrap().as_string().unwrap();
        let customer_id = i64::from_str(user_id_str).unwrap();
        let file_id = body.find_path(&["file_id"]).unwrap().as_i64().unwrap();
        let index = body.find_path(&["index"]).unwrap().as_i64().unwrap();
        let file_block_id = format!("{}_{}_{}", customer_id, file_id, index);
        let table = db.get_table("file_block").expect("file_block table not exits.");
        let mut file_block = body.clone();
        {
            let mut body_obj = file_block.as_object_mut().unwrap();
            body_obj.insert("id".to_string(), file_block_id.to_json());
            body_obj.insert("customer_id".to_string(), customer_id.to_json());
        }
        let op = Json::from_str("{}").unwrap();
        let rst = table.save(&file_block, &op);
        Result::Ok(Json::from_str("{}").unwrap())
    }
}

//delete the file
pub struct F04;

impl DataApi for F04 {

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
        let file_id = json_i64!(msg; "body", "id");

        let mut file_cond = json!("{}");
        json_set!(&mut file_cond; "customer_id"; customer_id);
        json_set!(&mut file_cond; "id"; file_id);

        let mut block_cond = json!("{}");
        json_set!(&mut block_cond; "customer_id"; customer_id);
        json_set!(&mut block_cond; "file_id"; file_id);

        let table = db.get_table("file").expect("file_block table not exits.");
        let table_block = db.get_table("file_block").expect("file_block table not exits.");

        let op = json!("{}");
        table.remove(&file_cond, &op);
        table_block.remove(&block_cond, &op);

        Result::Ok(json!("{}"))
    }
}

