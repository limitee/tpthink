use super::base::Ticket;

pub type ValidateResult = Result<(), i32>;

pub trait Validate: Send + Sync {
	
	fn validate(&self, ticket: &Ticket) -> ValidateResult;
}

struct ValidateSsq0000;

impl Validate for ValidateSsq0000 {
	
	fn validate(&self, ticket: &Ticket) -> ValidateResult {
		let amount = ticket.get_amount();
		
		Result::Ok(())
	}
}