use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

extern crate regex;
use self::regex::Regex;

use super::base::Ticket;
use super::base::Game;
use super::base::PlayType;
use super::base::BetType;
use super::base::GF;

pub type ValidateResult = Result<(i32), i32>;

pub trait Validate: Send + Sync {
	
	fn validate(&self, ticket: &Ticket, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult;
}

struct ValidateSsq1010;

impl Validate for ValidateSsq1010 {
	
	fn validate(&self, ticket: &Ticket, game:&Game, play_type:&PlayType, bet_type:&BetType) -> ValidateResult {
		let amount = ticket.get_amount();
		let price = play_type.get_price();
		let multiple = ticket.get_multiple();
		let number = ticket.get_number();
		let v: Vec<&str> = number.split(';').collect();
		let re = Regex::new(r"^([0-9]{2},){5},[0-9]{2}|[0-9]{2}$").unwrap();
		for num in &v {
            if !re.is_match(num) {
                return Result::Err(ErrCode::NumberIsWrong as i32);
            }
		}
		let count = v.len() as i32;
		let true_amount = count*price*multiple;
		let rst = {
			if amount == true_amount {
				Result::Ok((count))
			} else {
				Result::Err(ErrCode::AmountIsWrong as i32)
			}
		};
		rst
	}
}

pub struct ValidateFactory {
	map:BTreeMap<String, Box<Validate>>,
}

macro_rules! add_inter {
    ($o:expr, $k:expr, $v:expr) => {{
        $o.insert($k.to_string(), Box::new($v) as Box<Validate>);
    }}
}

impl ValidateFactory {
	
	pub fn new() -> ValidateFactory {
		let mut map = BTreeMap::new();
        add_inter!(map, "1001010", ValidateSsq1010);
        ValidateFactory {
            map:map,
        }
	}
	
	pub fn validate(&self, ticket:&Ticket) -> ValidateResult {
		let game = try!(GF.get_game_by_id(ticket.get_game_id()));
		let play_type = try!(game.get_play_type(ticket.get_play_type()));
		let bet_type = try!(play_type.get_bet_type(ticket.get_bet_type()));
		
		let key = format!("{}{}{}", ticket.get_game_id(), ticket.get_play_type(), ticket.get_bet_type());
		let validate = self.map.get(&key).unwrap();
		validate.validate(ticket, game, play_type, bet_type)
	}
}