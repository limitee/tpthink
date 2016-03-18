#[macro_use]
extern crate log;

extern crate elog;

fn main() {
    let _ = elog::init();
    info!(target:"main", "starting up");
    // ...
}