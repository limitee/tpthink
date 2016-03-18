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