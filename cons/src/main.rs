extern crate cons;
use cons::CONS;

fn main() {
    println!("{}", CONS.code_to_id("user_type", "admin").unwrap());
}
