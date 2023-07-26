mod cli;

use crate::cli::Cli;
use lazy_static::lazy_static;
use tracing::{info, Level};

lazy_static! {
    static ref ARGS: Cli = Cli::parse_opts();
}

fn main() {
    lazy_static::initialize(&ARGS);

    let level: tracing::Level;

    if ARGS.log.debug {
        // trace, debug, info, warn, error
        level = Level::TRACE;
    } else if ARGS.log.quiet {
        // warn, error
        level = Level::WARN;
    } else {
        // info, warn, error
        level = Level::INFO;
    }

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_max_level(level)
        .init();

    ARGS.validate();

    info!("{:?}", *ARGS)
}
