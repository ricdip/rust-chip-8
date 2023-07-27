use crate::ARGS;
use tracing::Level;

pub fn init() {
    let level: Level;

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
}
