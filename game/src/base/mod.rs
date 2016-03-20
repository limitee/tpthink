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
	
	pub fn get_amount(&self) -> i32 {
		self.amount
	}
	
	pub fn get_multiple(&self) -> i32 {
		self.multiple
	}
	
	pub fn get_number(&self) -> &str {
		&self.number
	}
}

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

///游戏玩法，单式，复试，胆托
pub struct PlayType {
	id:i32,
	price:i32,
	name: String,
}

impl PlayType {
	pub fn new(id:i32, price:i32, name:&str) -> PlayType {
		PlayType {
			id: id,
			price: price,
			name: name.to_string(),
		}
	}
	
	pub fn get_price(&self) -> i32 {
		self.price
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