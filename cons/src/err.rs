use std::fmt;
use std::collections::{HashMap, BTreeMap};

extern crate rustc_serialize;
use rustc_serialize::json::{ToJson, Json};

#[derive(Copy, Clone)]
pub struct ApiErr {
    pub id: i64,
    pub code: &'static str,
    pub des: &'static str,
}

impl fmt::Display for ApiErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.code, self.des)
    }
}

impl fmt::Debug for ApiErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.code, self.des)
    }
}

impl ToJson for ApiErr {
    
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("code".to_string(), self.code.to_json());
        map.insert("des".to_string(), self.des.to_json());
        Json::Object(map)
    }
}

pub struct ApiErrFactory {
    map:HashMap<&'static str, ApiErr>,
}

macro_rules! add_err {
    ($o:expr, $id:expr, $code:expr, $des:expr) => {{
        let err = ApiErr {
            id: $id,
            code: $code,
            des: $des,
        };
        $o.insert(err.code, err);
    }}
}

#[macro_export]
macro_rules! api_err {
    ($o:expr) => {{
        AEF.get_by_code($o)
    }}
}

impl ApiErrFactory {
    
    pub fn new() -> ApiErrFactory {
        let mut map = HashMap::new();
        add_err!(&mut map, 0, "success", "操作成功");
        add_err!(&mut map, 100, "digest_failure", "加密校验失败");
        add_err!(&mut map, 101, "api_not_exist", "接口不存在");
        ApiErrFactory {
            map:map
        }
    }

    pub fn get_by_code(&self, code:&str) -> ApiErr {
        let op = self.map.get(code); 
        op.unwrap().clone()
    }
}

lazy_static! {
    pub static ref AEF:ApiErrFactory = ApiErrFactory::new();
}
