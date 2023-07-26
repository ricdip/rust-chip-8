use std::path::PathBuf;
use clap::{Parser, Args};

const ARG_ROM_HELP: &str = "Path to CHIP-8 ROM file to run";
const ARG_ROM_VALUE_NAME: &str = "FILE";
const ARG_QUIET_HELP: &str = "Enable quiet logging";
const ARG_DEBUG_HELP: &str = "Enable debug logging";

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(short, long, required(true), value_name=ARG_ROM_VALUE_NAME, help=ARG_ROM_HELP)]
    pub rom: PathBuf,

    #[command(flatten)]
    pub log: Log
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct Log {
    #[arg(short, long, help=ARG_QUIET_HELP)]
    pub quiet: bool,
    #[arg(short, long, help=ARG_DEBUG_HELP)]
    pub debug: bool,
}

impl Cli {
    pub fn validate(&self) {
        let path = self.rom.as_path();

        match path.try_exists() {
            Ok(exists) => {
                if !exists {
                    panic!("rom file `{}` does not exist", path.to_str().unwrap())
                }
            }
            Err(e) => {
                panic!("rom file error: {e}")
            }
        }
    }

    pub fn parse_opts() -> Self {
      Self::parse()
    }
}