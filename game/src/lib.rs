#[macro_use]
extern crate lazy_static;

use std::collections::BTreeMap;

extern crate cons;
use cons::ErrCode;

pub mod base;
use base::Game;
use base::PlayType;
use base::BetType;

pub mod validate;