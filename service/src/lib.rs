use std::collections::BTreeMap;
use std::io::Read;

#[macro_use]
extern crate easy_util;
extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

extern crate util;
use self::util::DigestUtil;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

#[macro_use]
extern crate cons;
use cons::*;

extern crate time;

extern crate regex;
use self::regex::Regex;

pub mod inter;
use self::inter::{DataApi, KeyResult, CheckResult, RunResult};

mod sv_util;
use self::sv_util::{KeyHelper};

#[macro_use]
extern crate log;
extern crate elog;

mod ment;
use self::ment::user::*;

macro_rules! add_inter {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<DataApi>);
    }}
}

pub struct ApiFactory {
    map:BTreeMap<String, Box<DataApi>>,
}

impl ApiFactory {

    pub fn new() -> ApiFactory {
        let mut map = BTreeMap::new();
        ApiFactory {
            map:map,
        }
    }

    /**
     * get the digest key by head.
     */
    pub fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> KeyResult {
        let name = json_str!(head; "cmd");
        let api_op = self.map.get(name);
        match api_op {
            Some(api) => {
                api.get_key(db, head)
            },
            None => {
                Result::Err(api_err!("api_not_exist"))
            },
        }
    }

    /**
     * check the digest. If success return Some, else return None.
     */
    pub fn check_digest(&self, db:&DataBase<MyDbPool>, param:&BTreeMap<String, String>) -> Result<Json, ApiErr> {
        let head = param.get("head").unwrap();
        let head_node = json!(head);
        let digest = json_str!(&head_node; "digest");
        let time_stamp = json_str!(&head_node; "timeStamp");

        let key_rst = self.get_key(db, &head_node);
        key_rst.and_then(|key_info| {
            let body_str = param.get("body").unwrap();
            let digest_content = format!("{}{}{}", key_info.key, body_str, time_stamp);
            if digest == DigestUtil::md5(&digest_content) {
                let body_node = json!(body_str);
                let mut back_obj = json!("{}");
                json_set!(&mut back_obj; "head"; head_node.clone());
                json_set!(&mut back_obj; "body"; body_node);
                json_set!(&mut back_obj; "key"; key_info.key);
                Result::Ok(back_obj)
            }
            else
            {
                Result::Err(api_err!("digest_failure"))
            }
        })
    }

    pub fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> RunResult {
        let name = json_str!(msg; "head", "cmd");
        let api = self.map.get(name).unwrap();
        api.check(db, msg).and_then(|_|{
            api.run(db, msg)
        })
    }

    pub fn back(&self, msg:&Json, body:String) -> Json {
        let head = json_path!(msg; "head");
        let time = json_str!(head; "timeStamp");
        let key = json_str!(msg; "key");

        let digest_content = format!("{}{}{}", key, body, time);
        info!("{}", digest_content);
        let digest = DigestUtil::md5(&digest_content);
        info!("{}", digest);

        let mut back_head = head.clone();
        {
            json_set!(&mut back_head; "digest"; digest);
        }
        let mut back_obj = json!("{}");
        json_set!(&mut back_obj; "head"; back_head);
        json_set!(&mut back_obj; "body"; body);
        back_obj
    }

    pub fn back_err(&self, head:&Json, body:String) -> Json {
        let mut back_head = head.clone();
        {
            let time = json_str!(head; "timeStamp");
            let key = DigestUtil::empty_key();
            let digest_content = format!("{}{}{}", key, body, time);
            let digest = DigestUtil::md5(&digest_content);

            json_set!(&mut back_head; "digestType"; "md5-empty");
            json_set!(&mut back_head; "digest"; digest);
        }
        let mut back_obj = json!("{}");
        json_set!(&mut back_obj; "head"; back_head);
        json_set!(&mut back_obj; "body"; body);
        back_obj
    }
}

