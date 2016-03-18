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


//admin login
pub struct A01;

impl DataApi for A01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, mut head:&Json) -> Result<String, i32> {
        println!("{}", head);
        KeyHelper::from_db(db, head)
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let head = json_path!(msg;"head");
        let username = json_str!(head;"userId");
        let user_type = json_str!(head;"userType");
        let role_id = CONS.code_to_id("user_type", user_type).unwrap();
        let table = db.get_table("customer").expect("table not exists.");
        let cond = format!(r#"
            {{
                "username":"{}"
            }}
        "#, username);
        let fd_back = try!(table.find_by_str(&cond, "{}", "{}"));
        let rows = json_i64!(&fd_back; "rows");
        if rows > 0 {   //return the db object to client
            let user = json_path!(&fd_back;"data", "0");
            let id = json_i64!(user; "id");
            //check the st table if the id exits.
            let st_table = db.get_table("st").expect("st table not exists.");
            let st_cond = format!(r#"
                {{
                    "id":{}
                }}
            "#, id);
            let old_st = try!(st_table.find_by_str(&st_cond, "{}", "{}"));
            let old_st_rows = json_i64!(&old_st; "rows");
            //exits, update the last_active_time
            if(old_st_rows > 0)
            {
                let data = json_path!(&old_st; "data", "0");
                let mut st = json_str!(data; "st").to_string();
                let last_active_time = json_i64!(data; "last_active_time");
                let sec = time::get_time().sec;
                if sec - last_active_time > 1200 {
                    st = DigestUtil::random_key();
                }
                let doc = format!(r#"
                    {{
                        "$set":
                        {{
                            "last_active_time":{},
                            "st":"{}"
                        }}
                    }}
                "#, sec, st);
                let cond = format!(r#"
                    {{
                        "id":{}
                    }}
                "#, id);
                st_table.update_by_str(&cond, &doc, "{}");

                let body_str = format!(r#"
                    {{
                        "st":"{}",
                        "userId":{}
                    }}
                "#, st, id);
                Result::Ok(json!(&body_str))
            }
            else    //save the new st
            {
                let st = DigestUtil::random_key();
                let sec = time::get_time().sec;
                let sv_data = format!(r#"
                    {{
                        "st":"{}",
                        "last_active_time":{},
                        "id":{},
                        "role":{}
                    }}
                "#, st, sec, id, role_id);
                st_table.save_by_str(&sv_data, "{}");

                let body_str = format!(r#"
                    {{
                        "st":"{}",
                        "userId":{}
                    }}
                "#, st, id);
                Result::Ok(json!(&body_str))
            }
        }
        else
        {
            return Result::Err(ErrCode::UsernameIsWrong as i32)
        }
    }

}
