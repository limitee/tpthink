#[macro_use]
extern crate cons;
use cons::*;


#[test]
fn test() {
    let err = api_err!("success");
    assert_eq!(err.id, 0);
}
