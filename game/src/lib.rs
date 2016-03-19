use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

pub mod base;
pub mod validate;
use base::PlayType;
use base::BetType;

///系统中的一种游戏
///
///
pub struct Game {
	id:i32,
	code: String,
	name: String,
}

impl Game {
	///id为唯一标志,code为游戏简称，name为中文名称
	pub fn new(id:i32, code:&str, name:&str) -> Game {
		Game {
			id: id,
			code: code.to_string(),
			name: name.to_string(),
		}
	}
	
	///获得游戏id
	pub fn get_id(&self) -> i32 {
		self.id
	}
	
	///获得游戏名称
	pub fn get_name(&self) -> &str {
		&self.name
	}
	
	///获得游戏代码
	pub fn get_code(&self) -> &str {
		&self.code
	}
}

///工厂类，存储了所有游戏的详情
pub struct GameFactory {
	game_list:BTreeMap<i32, Game>,
}

impl GameFactory {
	pub fn new() -> GameFactory {
		let mut map = BTreeMap::new();
		let game = Game::new(100, "SSQ", "双色球");
		map.insert(100, game);
		GameFactory {
			game_list:map,
		}
    }
	
	
	///根据id获得&Game
	pub fn get_game_by_id(&self, id:i32) -> Result<&Game, i32> {
		let op:Option<&Game> = self.game_list.get(&id);
		op.ok_or(ErrCode::GameNotExists as i32)
	}
	
}