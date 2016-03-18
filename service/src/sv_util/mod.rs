extern crate util;
use self::util::DigestUtil;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

extern crate cons;
use self::cons::CONS;
use self::cons::ErrCode;

extern crate rustc_serialize;
use self::rustc_serialize::json::Json;
use self::rustc_serialize::json::ToJson;
use std::str::FromStr;

pub struct KeyHelper;

extern crate time;

impl KeyHelper {

    /**
     * get key from cache.
     */
    pub fn from_cache(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let user_id = json_i64!(head; "userId");
        let st_table = db.get_table("st").expect("st table not exists.");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let fd_back = try!(st_table.find_by_str(&cond, "{}", "{}"));
        let rows = json_i64!(&fd_back; "rows");
        if rows > 0 {
            let last_active_time = json_i64!(&fd_back; "data", "0", "last_active_time");
            let sec = time::get_time().sec;
            if sec - last_active_time > 120000 {
                Result::Err(ErrCode::TokenExpired as i32)
            }
            else
            {
                let st = json_str!(&fd_back; "data", "0", "st");
                Result::Ok(st.to_string())
            }
        }
        else
        {
            Result::Err(ErrCode::TokenExpired as i32)            
        }
    }

    pub fn active(db:&DataBase<MyDbPool>, head:&Json) {
        let user_id = json_i64!(head; "userId");
        let cond = format!(r#"
            {{
                "id":{}
            }}
        "#, user_id);
        let sec = time::get_time().sec;
        let doc = format!(r#"
        {{
            "$set":
                {{
                    "last_active_time":{}
                }}
            }}
        "#, sec);
        let st_table = db.get_table("st").expect("st table not exists.");
        let _ = st_table.update_by_str(&cond, &doc, "{}");
    }

    pub fn from_db(db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32> {
        let username = json_str!(head; "userId");
        let user_type = json_str!(head; "userType");

        let rst = CONS.code_to_id("user_type", &user_type);
        rst.and_then(|user_type_id| {
            let table = db.get_table("customer").expect("table not exists.");
            let cond = format!(r#"
               {{
                   "username":"{}",
                   "type":{}
               }}
            "#, username, user_type_id);
            let fd_back = try!(table.find_by_str(&cond, "{}", "{}"));
            let rows = json_i64!(&fd_back; "rows");
            if rows > 0 {   //return the db object to client
                let password = json_str!(&fd_back; "data", "0", "password");
                let digest = DigestUtil::md5(password);
                return Result::Ok(digest)
            }
            else
            {
               return Result::Err(ErrCode::UserInfoIsWrong as i32);
            }
        })
    }

}

