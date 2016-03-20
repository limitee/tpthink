use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

pub mod base;
use base::Game;
use base::PlayType;
use base::BetType;

pub mod validate;


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