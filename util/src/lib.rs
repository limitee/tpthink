extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

extern crate rustc_serialize;
use self::rustc_serialize::base64::{FromBase64, Config, ToBase64, CharacterSet, Newline, FromBase64Error};

extern crate time;

pub struct DigestUtil;

impl DigestUtil {

    pub fn md5(input:&str) -> String {
        let mut sh = Md5::new();
        sh.input_str(input);
        let out_str = sh.result_str();
        out_str
    }

    pub fn empty_key() -> String {
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()
    }

    /**
     * get a random key by time
     */
    pub fn random_key() -> String {
        let now = time::get_time();
        let str = format!("{:?}", &now);
        DigestUtil::md5(&str)
    }

    /**
     * get bytes from base64 string
     */
    pub fn base64_to_bytes(str:&str) -> Result<Vec<u8>, FromBase64Error> {
        str.from_base64() 
    }
}


pub struct TimeUtil;

impl TimeUtil {

    pub fn get_cur_second() -> i64 {
        let now = time::get_time();
        now.sec
    }
}

