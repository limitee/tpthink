pub mod base;
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
	pub fn new(id:i32, code:&str, name:&str) -> Game {
		Game {
			id: id,
			code: code.to_string(),
			name: name.to_string(),
		}
	}
}

pub struct GameFactory {
	
}