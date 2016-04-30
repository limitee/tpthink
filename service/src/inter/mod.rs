extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

extern crate cons;
use cons::ApiErr;

pub struct KeyInfo {
    pub key:String,
}

impl KeyInfo {

    //get KeyInfo from key
    pub fn from_key(key:String) -> KeyInfo {
        KeyInfo {
            key:key
        }
    }
}

pub type KeyResult = Result<KeyInfo, ApiErr>;

pub struct CheckInfo;

impl CheckInfo {
    
    //just a ok flag
    pub fn ok() -> CheckInfo {
        CheckInfo {
        }
    }

}

pub type CheckResult = Result<CheckInfo, ApiErr>;

pub type RunResult = Result<Json, ApiErr>;

///the data api
pub trait DataApi: Send + Sync {

    //check the body params
    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> CheckResult;

    //get the user's key
    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> KeyResult;

    //do the job
    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> RunResult;
}
