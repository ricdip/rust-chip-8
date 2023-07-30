//! Implementation of CHIP-8 (emulator execution)

use super::Chip8;
use rand::{rngs::StdRng, SeedableRng};
use std::{
    io, thread,
    time::{Duration, Instant},
};
use tracing::info;

impl Chip8 {
    /// Function that starts the CHIP-8 emulation
    ///
    /// # Arguments
    ///
    /// * `stepping` - Boolean that enables stepping execution (one cycle at time)
    /// * `seed` - Unsigned integer (u64) that is the seed for the random number generator
    ///
    /// # Panics
    ///
    /// The function panics if the ROM is not loaded or in case of illegal input during the stepping execution
    pub fn run(&mut self, stepping: bool, seed: u64) {
        if !self.rom_loaded {
            panic!("ROM is not loaded");
        }

        // init random number generator
        let mut rng = StdRng::seed_from_u64(seed);

        let mut instant: Instant;
        // TODO: break from loop
        // CHIP-8 clock is 500Hz, 500 heartbeats per second
        // an iteration of the game loop is called frame or tick
        // frame per second (fps) is how many loop iteration we have in 1 second
        // clock = frequency = cycles/seconds
        // seconds = cycles/clock
        let chip8_clock_time_seconds = 1.0 / 500.0;
        loop {
            instant = Instant::now();
            self.emulate_cycle(&mut rng);
            if self.draw {
                // TODO: drawing function
                info!("{}", self.dump_display());
            }
            let elapsed = instant.elapsed();

            let current_clock = 1.0 / elapsed.as_secs_f64();
            let current_clock_time_seconds = 1.0 / current_clock;

            // sleep for slowing down clock if necessary
            if current_clock_time_seconds > chip8_clock_time_seconds {
                thread::sleep(Duration::from_secs_f64(
                    current_clock_time_seconds - chip8_clock_time_seconds,
                ));
            }

            if stepping {
                let mut next = String::new();
                info!("[n] next, [q] quit");
                io::stdin().read_line(&mut next).unwrap();

                if next.trim() == "n" {
                    continue;
                } else if next.trim() == "q" {
                    break;
                } else {
                    panic!("illegal input");
                }
            }
        }
    }
}
