extern crate rustc_serialize;
use self::rustc_serialize::json::Json;

extern crate dc;
use self::dc::DataBase;
use self::dc::MyDbPool;

pub trait DataApi: Send + Sync {

    //check the body params
    fn check(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<i32, i32>;

    //get the user's key
    fn get_key(&self, db:&DataBase<MyDbPool>, head:&Json) -> Result<String, i32>;

    //do the job
    fn run(&self, db:&DataBase<MyDbPool>, msg:&Json) -> Result<Json, i32>;

}
