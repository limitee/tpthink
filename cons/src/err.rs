use std::error;
use std::fmt;

use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct ApiErr {
    id: i64,
    code: &'static str,
    des: &'static str,
}

pub struct ApiErrFactory {
    map:HashMap<&'static str, ApiErr>,
}

macro_rules! add_inter {
    ($o:expr, $id:expr, $code:expr, $des:expr) => {{
        let err = ApiErr {
            id: 0,
            code: "success",
            des: "操作成功",
        };
        $o.insert(err.code, err);
    }}
}

impl ApiErrFactory {
    
    pub fn new() -> ApiErrFactory {
        let mut map = HashMap::new();

        ApiErrFactory {
            map:map
        }
    }
}


