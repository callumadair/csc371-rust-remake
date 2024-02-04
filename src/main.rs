use clap::Parser;
use csc371_remake::_371pass::app;

fn main() -> () {
    let args = app::Args::parse();
    app::run(&args).expect("Bad things");
}
