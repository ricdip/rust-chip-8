//! CLI arguments parsing and validation

use clap::{Args, Parser};
use std::path::PathBuf;
use tracing::trace;

/// cli -f command help
const ARG_ROM_FILE_HELP: &str = "Path to CHIP-8 ROM file to run";

/// cli -f command value name
const ARG_ROM_FILE_VALUE_NAME: &str = "FILE";

/// cli -s command help
const ARG_STEPPING_HELP: &str = "Enable one step at time execution";

/// cli -r command help
const ARG_RANDOM_SEED_HELP: &str = "Random seed";

/// cli -r command value name
const ARG_RANDOM_SEED_VALUE_NAME: &str = "SEED";

/// cli -r command default value
const ARG_RANDOM_SEED_DEFAULT_VALUE: u64 = 10;

/// cli -q command help
const ARG_QUIET_HELP: &str = "Enable quiet logging";

/// cli -d command help
const ARG_DEBUG_HELP: &str = "Enable debug logging";

/// cli -t command help
const ARG_TRACE_HELP: &str = "Enable trace logging";

/// CLI arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    /// ROM file path
    #[arg(short = 'f', long = "rom-file", required(true), help=ARG_ROM_FILE_HELP, value_name=ARG_ROM_FILE_VALUE_NAME)]
    pub rom: PathBuf,

    /// Logging levels flags
    #[command(flatten)]
    pub log: Log,

    /// Stepping execution flag
    #[arg(short, long, help=ARG_STEPPING_HELP)]
    pub stepping: bool,

    /// Random seed
    #[arg(short, long, help=ARG_RANDOM_SEED_HELP, value_name=ARG_RANDOM_SEED_VALUE_NAME, default_value_t=ARG_RANDOM_SEED_DEFAULT_VALUE)]
    pub random_seed: u64,
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

    /// Trace logging flag
    #[arg(short, long, help=ARG_TRACE_HELP)]
    pub trace: bool,
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
