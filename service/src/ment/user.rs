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

use super::super::inter::{DataApi, KeyResult, CheckResult, RunResult};
use super::super::sv_util::{KeyHelper};

//user register
pub struct U01;

impl DataApi for U01 {

    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> KeyResult {
        Result::Ok(DigestUtil::empty_key())
    }

    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> CheckResult {
        Result::Ok(0)
    }

    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> RunResult {
        Result::Ok(Json::from_str("{}").unwrap())
    }
}
