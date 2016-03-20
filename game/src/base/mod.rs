use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

pub struct Ticket {
	game_id: i32,
	play_type: i32,
	bet_type: i32,
	amount: i32,		//金额
	multiple: i32,	//倍数
	number: String,
}

impl Ticket {
	
	pub fn new(game_id:i32, play_type:i32, bet_type:i32, multiple:i32, amount:i32, number:&str) -> Ticket {
		Ticket {
			game_id:game_id,
			play_type: play_type,
			bet_type: bet_type,
			multiple: multiple,
			amount:amount,
			number:number.to_string(),
		}
	}
	
	pub fn get_game_id(&self) -> i32 {
		self.game_id
	}
	
	pub fn get_amount(&self) -> i32 {
		self.amount
	}
	
	pub fn get_multiple(&self) -> i32 {
		self.multiple
	}
	
	pub fn get_number(&self) -> &str {
		&self.number
	}
	
	pub fn get_play_type(&self) -> i32 {
		self.play_type
	}
	
	pub fn get_bet_type(&self) -> i32 {
		self.bet_type
	}
}

///系统中的一种游戏
///
///
pub struct Game {
	id:i32,
	code: String,
	name: String,
	map: BTreeMap<i32, PlayType>,
}

impl Game {
	///id为唯一标志,code为游戏简称，name为中文名称
	pub fn new(id:i32, code:&str, name:&str, map:BTreeMap<i32, PlayType>) -> Game {
		Game {
			id: id,
			code: code.to_string(),
			name: name.to_string(),
			map: map,
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
	
	///根据玩法id获得玩法
	pub fn get_play_type(&self, play_type_id:i32) -> Result<&PlayType, i32> {
		let op:Option<&PlayType> = self.map.get(&play_type_id);
		op.ok_or(ErrCode::PlayTypeNotExists as i32)
	}
}

///游戏玩法，单式，复试，胆托
pub struct PlayType {
	id:i32,
	price:i32,
	name: String,
	map: BTreeMap<i32, BetType>,
}

impl PlayType {
	pub fn new(id:i32, price:i32, name:&str, map: BTreeMap<i32, BetType>) -> PlayType {
		PlayType {
			id: id,
			price: price,
			name: name.to_string(),
			map: map,
		}
	}
	
	pub fn get_price(&self) -> i32 {
		self.price
	}
	
	///根据id获得投注方式
	pub fn get_bet_type(&self, bet_type_id:i32) -> Result<&BetType, i32> {
		let op:Option<&BetType> = self.map.get(&bet_type_id);
		op.ok_or(ErrCode::BetTypeNotExists as i32)
	}
}

///投注方式，如标准
pub struct BetType {
	id:i32,
	name: String,
}

impl BetType {
	
	pub fn new(id:i32, name:&str) -> BetType {
		BetType {
			id: id,
			name: name.to_string(),
		}
	}
}


///工厂类，存储了所有游戏的详情
pub struct GameFactory {
	game_list:BTreeMap<i32, Game>,
}

fn get_ssq_game() -> Game {
	let mut map = BTreeMap::new();
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "标准"));
	map.insert(10, PlayType::new(10, 200, "单式", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "标准"));
	map.insert(11, PlayType::new(11, 200, "复式", bet_map));
	
	let mut bet_map = BTreeMap::new();
	bet_map.insert(10, BetType::new(10, "标准"));
	map.insert(12, PlayType::new(12, 200, "胆托", bet_map));
	
	Game::new(100, "SSQ", "双色球", map)
}

impl GameFactory {
	
	pub fn new() -> GameFactory {
		let mut map = BTreeMap::new();
		let game = get_ssq_game();
		map.insert(game.get_id(), game);
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

lazy_static! {
    pub static ref GF:GameFactory = GameFactory::new();
}