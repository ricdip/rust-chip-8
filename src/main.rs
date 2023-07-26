mod cli;

use crate::cli::Cli;
use lazy_static::lazy_static;

lazy_static!{
    static ref ARGS: Cli = Cli::parse_opts();
}

fn main() {
    lazy_static::initialize(&ARGS);
    ARGS.validate();

    println!("{:?}", *ARGS);
}
