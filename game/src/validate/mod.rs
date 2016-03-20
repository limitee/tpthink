use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

use super::base::Ticket;
use super::base::Game;
use super::base::PlayType;
use super::base::BetType;
use super::base::GF;

pub type ValidateResult = Result<(), i32>;

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
		for num in &v {
			println!("{}", num);
		}
		let true_amount = v.len() as i32*price*multiple;
		let rst = {
			if amount == true_amount {
				Result::Ok(())
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
	
	pub fn validate(ticket:&Ticket) -> ValidateResult {
		let game = try!(GF.get_game_by_id(ticket.get_game_id()));
		let play_type = try!(game.get_play_type(ticket.get_play_type()));
		Result::Ok(())
	}
}