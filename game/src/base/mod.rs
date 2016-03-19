pub struct Ticket {
	game_id: i32,
	play_type: i32,
	bet_type: i32,
	amount: i32,
	number: String,
}

impl Ticket {
	
	pub fn new(game_id:i32, play_type:i32, bet_type:i32, amount:i32, number:&str) -> Ticket {
		Ticket {
			game_id:game_id,
			play_type: play_type,
			bet_type: bet_type,
			amount:amount,
			number:number.to_string(),
		}
	}
	
	pub fn get_amount(&self) -> i32 {
		self.amount
	}
}


///游戏玩法，单式，复试，胆托
pub struct PlayType {
	id:i32,
	name: String,
}

impl PlayType {
	pub fn new(id:i32, name:&str) -> PlayType {
		PlayType {
			id: id,
			name: name.to_string(),
		}
	}
}

///投注方式，如标准
pub struct BetType {
	id:i32,
	name: String,
}

impl BetType {
	
	pub fn new(id:i32, name:&str) -> PlayType {
		PlayType {
			id: id,
			name: name.to_string(),
		}
	}
	
}