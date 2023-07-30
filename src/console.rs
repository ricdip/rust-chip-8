//! Console logging

use crate::ARGS;
use tracing::{debug, Level};

/// Initialize tracing console logging
pub fn init() {
    let level: Level;
    let mut subscriber = tracing_subscriber::fmt();

    // set logging level from args
    if ARGS.log.trace {
        // enable trace, debug, info, warn, error levels
        level = Level::TRACE;
        subscriber = subscriber.with_target(true).with_thread_ids(true);
    } else if ARGS.log.debug {
        // enable debug, info, warn, error levels
        level = Level::DEBUG;
        subscriber = subscriber.with_target(true);
    } else if ARGS.log.quiet {
        // enable warn, error levels
        level = Level::WARN;
    } else {
        // enable info, warn, error levels
        level = Level::INFO;
    }

    // initialize tracing logging
    subscriber
        .with_level(true)
        .with_writer(std::io::stderr)
        .with_ansi(true)
        .with_max_level(level)
        .init();

    debug!("logging level set: {}", level.as_str());
}
