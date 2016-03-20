extern crate game;
use game::validate::ValidateFactory;
use game::base::Ticket;

fn main() {
	let ticket = Ticket::new(100, 10, 10, 1, 200, "01,02,03,04,05,06|01");
	let VF = ValidateFactory::new();
	let rst = VF.validate(&ticket);
	assert!(rst.is_ok());
	assert!(rst.unwrap() == (1));
}
