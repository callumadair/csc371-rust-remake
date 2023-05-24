pub mod _371pass;
mod wallet;
mod category;
mod item;

extern crate getopts;

use std::io::Error;

fn main() -> Result<(), Error> {
    return _371pass::app::run();
}
