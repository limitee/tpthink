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

//admin get document list
pub struct AD01;

impl DataApi for AD01 {

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

        let table = db.get_table("document").expect("document table not exists.");
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

//admin add document
pub struct AD02;

impl DataApi for AD02 {

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

        let table = db.get_table("document").expect("document table not exists.");
        let doc = json_path!(msg; "body", "data");
        let id_node = doc.find("id");
        
        match id_node {
        		Some(id) => {
        			let id = id.as_i64().unwrap();
        			let title = json_str!(doc; "title");
        			let content = json_str!(doc; "content");
        			let mut cond = json!("{}");
        			json_set!(&mut cond; "id"; id);
        			
        			let mut doc = json!("{}");
        			let mut set_data = json!("{}");
        			json_set!(&mut set_data; "title"; title);
        			json_set!(&mut set_data; "content"; content);
        			json_set!(&mut doc; "$set"; set_data);
        			
        			let op = json!("{}");
        			let rst = table.update(&cond, &doc, &op);
        			rst.and_then(|_|{
		        		Result::Ok(json!("{}"))			
		        })
        		},
        		None => {
        			let mut doc = doc.clone();
		        let now = time::get_time();
		        json_set!(&mut doc; "create_time"; now.sec);
		        json_set!(&mut doc; "customer_id"; customer_id);
		        
		        let rst = table.save(&doc, &json!("{}"));
		        rst.and_then(|_|{
		        		Result::Ok(json!("{}"))			
		        })			
        		}
        }
        //info!("{}", doc);
        
    }
}

//admin get document by id
pub struct AD03;

impl DataApi for AD03 {

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

        let table = db.get_table("document").expect("document table not exists.");
        let doc_id = json_i64!(msg; "body", "id");
        let mut cond = json!("{}");
        json_set!(&mut cond; "id"; doc_id);
        json_set!(&mut cond; "customer_id"; customer_id);
        
        let doc = json!("{}");
        let op = json!("{}");
        let rst = table.find(&cond, &doc, &op);
        rst.and_then(|json| {
        		Result::Ok(json)	
        	})
    }
}