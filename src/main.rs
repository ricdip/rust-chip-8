mod cli;
mod console;

use crate::cli::Cli;
use lazy_static::lazy_static;
use tracing::info;

lazy_static! {
    static ref ARGS: Cli = Cli::parse_opts();
}

fn main() {
    lazy_static::initialize(&ARGS);
    ARGS.validate();
    console::init();

    info!("{:?}", *ARGS)
}
