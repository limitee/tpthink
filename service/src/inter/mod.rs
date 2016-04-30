extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

extern crate cons;
use cons::ApiErr;

pub type KeyInfo = (String);

pub type KeyResult = Result<KeyInfo, ApiErr>;

pub type CheckInfo = (String);

pub type CheckResult = Result<CheckInfo, ApiErr>;

pub type RunResult = Result<Json, ApiErr>;

///the data api
pub trait DataApi: Send + Sync {

    //check the body params
    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> KeyResult;

    //get the user's key
    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> CheckResult;

    //do the job
    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> RunResult;
}
