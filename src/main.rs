pub mod _371pass;
mod category;
mod item;
mod wallet;

use crate::_371pass::app;
use clap::Parser;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args = app::Args::parse();
    app::run(&args)
}
