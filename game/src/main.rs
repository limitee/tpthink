extern crate game;
use game::validate::ValidateFactory;
use game::base::Ticket;

extern crate cons;
use cons::ErrCode;

fn main() {
	let VF = ValidateFactory::new();
	
	let ticket = Ticket::new(100, 10, 10, 1, 200, "01,02,03,04,05,06|01");
	let rst = VF.validate(&ticket);
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (1));
	
	let ticket = Ticket::new(100, 10, 10, 1, 400, "01,02,03,04,05,06|01;01,02,03,04,05,26|10");
	let rst = VF.validate(&ticket);
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (2));
	
	let ticket = Ticket::new(100, 10, 10, 1, 1200, "01,02,03,04,05,06|01;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10;01,02,03,04,05,26|10");
	let rst = VF.validate(&ticket);
	assert!(rst.is_err());
	assert!(rst.err().unwrap() == ErrCode::CountBtFive as i32);
	
}
