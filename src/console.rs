//! Console logging

use crate::ARGS;
use tracing::Level;

/// Initialize tracing console logging
pub fn init() {
    let level: Level;

    // set logging level from args
    if ARGS.log.trace {
        // enable trace, debug, info, warn, error levels
        level = Level::TRACE;
    } else if ARGS.log.debug {
        // enable debug, info, warn, error levels
        level = Level::DEBUG;
    } else if ARGS.log.quiet {
        // enable warn, error levels
        level = Level::WARN;
    } else {
        // enable info, warn, error levels
        level = Level::INFO;
    }

    // initialize tracing logging
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_max_level(level)
        .init();
}
