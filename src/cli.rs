//! CLI arguments parsing and validation

use clap::{Args, Parser};
use std::path::PathBuf;
use tracing::trace;

/// cli -r command help
const ARG_ROM_HELP: &str = "Path to CHIP-8 ROM file to run";
/// cli -r command value
const ARG_ROM_VALUE_NAME: &str = "FILE";
/// cli -q command help
const ARG_QUIET_HELP: &str = "Enable quiet logging";
/// cli -d command help
const ARG_DEBUG_HELP: &str = "Enable verbose logging";

/// CLI arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// ROM file path arg
    #[arg(short, long, required(true), value_name=ARG_ROM_VALUE_NAME, help=ARG_ROM_HELP)]
    pub rom: PathBuf,

    /// Logging levels flags
    #[command(flatten)]
    pub log: Log,
}

/// Log group arguments structure
#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct Log {
    /// Quiet logging flag
    #[arg(short, long, help=ARG_QUIET_HELP)]
    pub quiet: bool,

    /// Debug logging flag
    #[arg(short, long, help=ARG_DEBUG_HELP)]
    pub debug: bool,
}

impl Cli {
    /// Validates the CLI arguments
    pub fn validate(&self) {
        // validate ROM path
        trace!("validate: start");

        let path = self.rom.as_path();

        match path.try_exists() {
            Ok(exists) => {
                if !exists {
                    panic!("rom file `{}` does not exist", path.to_str().unwrap())
                }
            }
            Err(e) => {
                // check file error occurred
                panic!("rom file error: {e}")
            }
        }

        trace!("validate: exit");
    }

    /// Parses and returns CLI arguments
    pub fn parse_opts() -> Self {
        trace!("parse_opts: start");
        trace!("parse_opts: end");

        // parse cli args
        Self::parse()
    }
}
