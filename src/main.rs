//! # rust-chip-8
//!
//! `rust-chip-8` is a simple implementation of CHIP-8 written in Rust for fun and training purposes

mod chip8;
mod cli;
mod console;

use crate::cli::Cli;
use chip8::Chip8;
use lazy_static::lazy_static;
use std::panic;
use tracing::{error, info};

// static that contains CLI args
lazy_static! {
    static ref ARGS: Cli = Cli::parse_opts();
}

fn main() {
    // initialize args
    lazy_static::initialize(&ARGS);
    // initialize console
    console::init();

    // panics will use tracing::error for printing panic info
    // and will exit with code 1
    panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info.to_string());
        std::process::exit(1);
    }));

    // validate args
    ARGS.validate();

    // loop
    let chip8 = Chip8::new();

    info!("{}", chip8);
}
