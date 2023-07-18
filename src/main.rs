pub mod _371pass;
mod wallet;
mod category;
mod item;

use std::io::Error;
use clap::Parser;
use crate::_371pass::app;

fn main() -> Result<(), Error> {
    let args = app::Args::parse();
    return app::run(&args);
}
