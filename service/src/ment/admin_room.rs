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

//get file list
pub struct AR01;

impl DataApi for AR01 {

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
        let table = db.get_table("file").expect("file table not exists.");
        let c_data = table.count_by_str("{}", "{}").unwrap();
        println!("{}", c_data);
        let mut data = try!(table.find_by_str("{}", "{}", "{}"));
        {
            let mut data_obj = data.as_object_mut().unwrap();
            data_obj.insert("count".to_string(), json_i64!(&c_data; "data", "0", "count").to_json());
        }
        Result::Ok(data)
    }

}

//get room type list
pub struct ART01;

impl DataApi for ART01 {

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
        let table = db.get_table("room_type").expect("st table not exists.");
        let cond = json_str!(msg; "body", "cond");
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
        let c_data = try!(table.count_by_str(cond, "{}"));
        let mut data = try!(table.find(&json!(cond), &json!("{}"), &json_op));
        {
            let count = json_i64!(&c_data; "data", "0", "count");
            json_set!(&mut data;"count";count);
        }
        Result::Ok(data)
    }

}

//add room type
pub struct ART02;

impl DataApi for ART02 {

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
        let table = db.get_table("room_type").expect("st table not exists.");
        let mut data = json_path!(msg; "body", "data").clone();
        let now = time::get_time();
        json_set!(&mut data; "create_time"; now.sec);
        let op = json!("{}");
        let ret = table.save(&data, &op);
        ret
    }

}

//move room type
pub struct ART03;

impl DataApi for ART03 {

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
        let table = db.get_table("room_type").expect("st table not exists.");
        let book_type_id = json_i64!(msg; "body", "id");
        let up_or_down = json_i64!(msg; "body", "up_or_down");
        let index = json_i64!(msg; "body", "index");
        let target_set = {
            let cond;
            let sort;
            if up_or_down > 0 {
                cond = format!(r#"
                    {{
                        "index": {{
                            "$gt":{} 
                        }}
                    }}
                "#, index);
                sort = r#"
                    [{
                        "index": 1
                    }]
                "#;
            } else {
                cond = format!(r#"
                    {{
                        "index": {{
                            "$lt":{} 
                        }}
                    }}
                "#, index);
                sort = r#"
                    [{
                        "index": -1
                    }]
                "#;
            }
            let mut op = json!("{}");
            json_set!(op; "offset"; 0);
            json_set!(op; "limit"; 1);
            json_set!(op; "sort"; json!(sort));
            let data = json!("{}");
            let cond_json = json!(&cond);
            table.find_one(&cond_json, &data, &op)
        };
        if target_set.is_err() {
            return Result::Ok(json!("{}"));
        }
        let set = target_set.unwrap();
        let t_index = json_i64!(&set; "index");
        let rst = {
            let cond = format!(r#"
                {{
                    "index":{}
                }}
            "#, t_index);
            let doc = format!(r#"
                {{
                    "$set":
                    {{
                        "index":{}
                    }}
                }}
            "#, index);
            let op = r#"
                {
                   "ret": 
                   {
                       "id": 1
                   }
                }
            "#;
            let data = try!(table.update_by_str(&cond, &doc, &op));
            let row = json_i64!(&data; "rows");
            row
        };
        if rst > 0 {
            let cond = format!(r#"
                {{
                    "id":{}
                }}
            "#, book_type_id);
            let doc = format!(r#"
                {{
                    "$set":
                    {{
                        "index":{}
                    }}
                }}
            "#, t_index);
            table.update_by_str(&cond, &doc, "{}");
        }
        Result::Ok(json!("{}"))
    }

}

//delete room type
pub struct ART04;

impl DataApi for ART04 {

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
        let table = db.get_table("room_type").expect("st table not exists.");
        let type_id = json_i64!(msg; "body", "id");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, type_id);
        table.remove_by_str(&cond, "{}");
        Result::Ok(json!("{}"))
    }

}

//get room order list
pub struct ARO01;

impl DataApi for ARO01 {

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
        let table = db.get_table("room_order").expect("st table not exists.");
        let cond = json_str!(msg; "body", "cond");
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
        let c_data = try!(table.count_by_str(cond, "{}"));
        let mut data = try!(table.find(&json!(cond), &json!("{}"), &json_op));
        {
            let count = json_i64!(&c_data; "data", "0", "count");
            json_set!(&mut data;"count";count);
        }
        Result::Ok(data)
    }

}

