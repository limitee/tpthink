#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;
use std::sync::{Arc};

pub enum ErrCode {
    Success = 0,

    DigestFailure,
    ApiNotExits,
    TokenExpired,
    ValueNotExist,

    UsernameIsNull,
    UsernameWrongPattern,
    UsernameIsWrong,

    PasswordIsNull,
    PasswordWrongPattern,

    UserInfoIsWrong,
	
	GameNotExists,	//游戏不存在
	AmountIsWrong,	//金额错误
	PlayTypeNotExists,	//玩法不存在
	BetTypeNotExists,	//投注方式不存在
	NumberIsWrong,	//号码格式错误
}

pub struct ConsNode {
    id:i32,
    code:String,
    desc:String,
}

impl ConsNode {
	
	pub fn new(id:i32, code:&str, desc:&str) -> ConsNode {
		ConsNode {
			id:id,
			code:code.to_string(),
			desc:desc.to_string(),
		}
	}
	
}

pub struct Cons {
    code_data:BTreeMap<String, Arc<ConsNode>>,
    id_data:BTreeMap<i32, Arc<ConsNode>>,
}


impl Cons {

    pub fn from_vec(vec:Vec<Arc<ConsNode>>) -> Cons {
        let mut code_data:BTreeMap<String, Arc<ConsNode>> = BTreeMap::new();
        let mut id_data:BTreeMap<i32, Arc<ConsNode>> = BTreeMap::new();
        for node in vec {
            code_data.insert(node.code.clone(), node.clone());
            id_data.insert(node.id, node);
        }
        Cons {
            code_data: code_data,
            id_data: id_data,
        }
    }

}

pub struct ConsFactory {
    cons:BTreeMap<String, Cons>,
}

macro_rules! get_node {
    ($o:expr, $k:expr, $v:expr) => {{
        Arc::new(ConsNode::new($o, $k, $v))
    }}
}


impl ConsFactory {

    pub fn new() -> ConsFactory {
        let mut cons:BTreeMap<String, Cons> = BTreeMap::new();
        let user_type_vec = vec![
            get_node!(100, "guest", "游客"),
            get_node!(200, "normal", "普通用户"),
            get_node!(300, "hotel", "餐馆"),
            get_node!(900, "admin", "管理员"),
        ];
        cons.insert("user_type".to_string(), Cons::from_vec(user_type_vec));
        let file_type_vec = vec![
            get_node!(-1, "unknown", "未知"),
            get_node!(100, "text/xml", "xml"),
            get_node!(200, "text/plain", "txt"),
            get_node!(300, "image/png", "png"),
        ];
        cons.insert("file_type".to_string(), Cons::from_vec(file_type_vec));

        let desk_status_vec = vec![
            get_node!(0, "idle", "空闲"),
            get_node!(100, "eat", "就餐中"),
        ];
        cons.insert("desk_status".to_string(), Cons::from_vec(desk_status_vec));

        let order_status_vec = vec![
            get_node!(0, "eat", "就餐中"),
            get_node!(100, "finish", "结账中"),
        ];
        cons.insert("order_status".to_string(), Cons::from_vec(order_status_vec));

        ConsFactory{
            cons:cons,
        }
    }

    pub fn by_id(&self, name:&str, id:i32) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.id_data.get(&id)
    }

    pub fn code_to_id(&self, name:&str, code:&str) -> Result<i32, i32> {
        //println!("the name is {}.", name);
        //println!("the code is {}.", code);
        let op = self.by_code(name, code);
        match op {
            Some(x) => {
                Result::Ok((**x).id)
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }

    pub fn by_code(&self, name:&str, code:&str) -> Option<&Arc<ConsNode>> {
        let cons:&Cons = self.cons.get(name).unwrap();
        cons.code_data.get(code)
    }

    pub fn id_to_code(&self, name:&str, id:i32) -> Result<String, i32> {
        let op = self.by_id(name, id);
        match op {
            Some(x) => {
                Result::Ok((**x).code.clone())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }
    
    pub fn id_to_desc(&self, name:&str, id:i32) -> Result<&str, i32> {
        let op = self.by_id(name, id);
        match op {
            Some(x) => {
                Result::Ok((**x).desc.as_str())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }
	
	pub fn code_to_desc(&self, name:&str, code:&str) -> Result<&str, i32> {
        let op = self.by_code(name, code);
        match op {
            Some(x) => {
                Result::Ok((**x).desc.as_str())
            },
            None => {
                Result::Err(ErrCode::ValueNotExist as i32)
            },
        }
    }
	
}

lazy_static! {
    pub static ref CONS:ConsFactory = ConsFactory::new();
}


