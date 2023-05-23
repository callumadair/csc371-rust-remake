pub mod _371pass;
mod wallet;
mod category;
mod item;

extern crate getopts;

use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    return _371pass::app::run(args);
}
