#[macro_use]
extern crate log;

extern crate elog;

fn main() {
    elog::init();
    info!(target:"main", "starting up");
    // ...
}