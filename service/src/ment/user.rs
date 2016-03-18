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

//user register
pub struct U01;

impl DataApi for U01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        Result::Ok(DigestUtil::empty_key())
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32> {
        let body = json_path!(msg; "body");
        let username_node = body.find("username");
        match username_node {
            Some(x) => {
                if x.as_string().is_none() {
                    return Result::Err(ErrCode::UsernameWrongPattern as i32);
                }
                else
                {
                    let username = x.as_string().unwrap();
                    println!("{}", username);
                    let re = Regex::new(r"^[a-z|A-Z]{1}[a-z|A-Z|1-9]{5, 19}$").unwrap();
                    if !re.is_match(username) {
                        return Result::Err(ErrCode::UsernameWrongPattern as i32);
                    }
                }
            },
            None => {
                return Result::Err(ErrCode::UsernameIsNull as i32);
            },
        }
        let password_node = body.find("password");
        match password_node {
            Some(x) => {
                if x.as_string().is_none() {
                    return Result::Err(ErrCode::PasswordWrongPattern as i32);
                }
                else
                {
                    let password = x.as_string().unwrap();
                    let re = Regex::new(r"^[a-z|A-Z|1-9|#|@|!]{6, 20}$").unwrap();
                    if !re.is_match(password) {
                        return Result::Err(ErrCode::PasswordWrongPattern as i32);
                    }
                }
            },
            None => {
                return Result::Err(ErrCode::PasswordIsNull as i32);
            }
        }
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32> {
        let table = db.get_table("customer").expect("table not exists.");
        let username = json_str!(msg; "body", "username");
        let password = json_str!(msg; "body", "password");

        let mut save_obj = json!("{}");
        json_set!(&mut save_obj; "username"; username);
        json_set!(&mut save_obj; "password"; password);
        let now = time::get_time();
        json_set!(&mut save_obj; "reg_time"; now.sec);
        table.save(&save_obj, &json!("{}"));
        Result::Ok(Json::from_str("{}").unwrap())
    }

}
