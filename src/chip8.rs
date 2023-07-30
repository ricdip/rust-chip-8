//! Implementation of CHIP-8

use core::panic;
use std::{
    fmt::Display,
    fs::File,
    io::{self, Read},
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use tracing::{debug, info, trace};

/// max RAM memory
const MAX_MEMORY_SIZE: usize = 4096;

/// display width
const DISPLAY_WIDTH: usize = 64;

/// display height
const DISPLAY_HEIGTH: usize = 32;

/// display size: (width x height) = (64 x 32)
const MAX_DISPLAY_SIZE: usize = DISPLAY_WIDTH * DISPLAY_HEIGTH;

/// max stack levels
const MAX_STACK_SIZE: usize = 16;

/// max V size
const V_SIZE: usize = 16;

/// CHIP-8 fontset.
/// Each font is 2 nibbles (or half-bytes) = 1 bytes = 8 bits
const CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// CHIP-8 representation
pub struct Chip8 {
    /// Boolean set to true if ROM has been loaded into memory, false otherwise
    rom_loaded: bool,

    /// CHIP-8 has 35 opcodes that are all 2 bytes = 16 bits long
    opcode: u16,

    /// CHIP-8 has 4KB = 4096 bytes of RAM memory in total.
    /// The fontset should be loaded at memory locations 0-80 (0x00-0x50).
    /// The program should be loaded at memory 512-onwards (0x200-onwards)
    memory: [u8; MAX_MEMORY_SIZE],

    /// CHIP-8 has 15 8-bit general purpose CPU registers named V0-VE.
    /// The 16th register (VF) is used for 'carry flag'.
    /// every CPU register has 8 bit = 1 byte length
    v: [u8; V_SIZE],

    /// CHIP-8 has one 16-bit Index Register (I) that points at locations in memory
    i: u16,

    /// CHIP-8 has one 16-bit Program Counter (PC) that points at the current instruction in memory
    pc: u16,

    /// CHIP-8 has a black and white graphics and the screen has a total of 2048 pixels (64 x 32).
    /// We can implement this with an array of booleans that holds the pixel state (1 or 0)
    display: [bool; MAX_DISPLAY_SIZE],

    /// CHIP-8 draw flag. If flag is set to true, redraw screen
    draw: bool,

    /// CHIP-8 has a stack used to remember the current location
    /// before a jump is performed.
    /// (CHIP-8 instruction set has opcodes that allow the
    /// program to jump to a certain address or call a subroutine)
    /// So, anytime we perform a jump or call a subroutine, we
    /// store the PC in the stack before proceeding.
    /// the stack stores 16-bit addresses (2 bytes = 16 bits)
    /// and has 16 levels of stack. In order to remember which level
    /// of the stack is used, we need to implement a stack pointer (SP)
    stack: [u16; MAX_STACK_SIZE],

    /// CHIP-8 Stack Pointer (SP) used to remember which level of the stack is used (16 levels: 0-15)
    sp: u8,

    /// CHIP-8 has two 8-bit timer registers that count at 60Hz
    /// when these registers are set with a value > 0, they
    /// will count down until 0
    timers: Timers,
}

/// Structure that contains CHIP-8 delay_timer and sound_timer
struct Timers {
    /// 8-bit Delay Timer which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0
    delay_timer: u8,
    /// 8-bit Sound Timer which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0
    sound_timer: u8,
}

impl Chip8 {
    /// Resets CHIP-8 instance
    fn reset(&mut self) {
        trace!("Chip8::reset: start");
        debug!("before reset: {}", self);

        // clear rom_loaded flag
        self.rom_loaded = false;

        // reset current opcode
        self.opcode = 0;

        // clear memory
        for i in 0..MAX_MEMORY_SIZE {
            self.memory[i] = 0;
        }

        // clear registers V0-VF
        for i in 0..V_SIZE {
            self.v[i] = 0;
        }

        // reset I
        self.i = 0;

        // PC starts at 0x200
        self.pc = 0x200;

        // clear display
        for i in 0..MAX_DISPLAY_SIZE {
            self.display[i] = false;
        }

        // reset draw flag
        self.draw = false;

        // clear stack
        for i in 0..MAX_STACK_SIZE {
            self.stack[i] = 0;
        }

        // reset SP
        self.sp = 0;

        // reload fontset into memory (0x00-0x50)
        self.load_fontset();

        // reset timers
        self.timers.delay_timer = 0;
        self.timers.sound_timer = 0;

        debug!("after reset: {}", self);
        trace!("Chip8::reset: exit");
    }

    /// Loads CHIP-8 fontset into memory at locations 0x00-0x50
    fn load_fontset(&mut self) {
        trace!("Chip8::load_fontset: start");

        // load fontset into memory (0x00-0x50)
        for i in 0x00..0x50 {
            self.memory[i] = CHIP8_FONTSET[i];
        }

        trace!("Chip8::load_fontset: exit");
    }

    /// Returns a new CHIP-8 instance ready to load a new ROM file
    pub fn new() -> Self {
        trace!("Chip8::new: start");

        // create new chip8 instance
        let mut chip8 = Self {
            rom_loaded: false,
            opcode: 0,
            memory: [0; MAX_MEMORY_SIZE],
            v: [0; V_SIZE],
            i: 0,
            pc: 0x200,
            display: [false; MAX_DISPLAY_SIZE],
            draw: false,
            stack: [0; MAX_STACK_SIZE],
            sp: 0,
            timers: Timers {
                delay_timer: 0,
                sound_timer: 0,
            },
        };
        // load fontset
        chip8.load_fontset();

        debug!("new chip8 instance: {}", chip8);
        trace!("Chip8::new: exit");

        // return created instance
        chip8
    }

    /// Loads a ROM file into the memory of the current CHIP-8 instance
    ///
    /// # Arguments
    ///
    /// * `file` - The PathBuf reference that holds the path to the ROM file
    ///
    /// # Panics
    ///
    /// The function panics in case of errors during opening and reading of the ROM file
    pub fn load_rom(&mut self, file: &PathBuf) {
        trace!("Chip8::load_rom: start");

        // opening file
        let mut rom = match File::open(file.as_path()) {
            Ok(f) => f,
            Err(e) => {
                panic!("opening rom file: {e}")
            }
        };
        // reading file
        let mut contents = Vec::new();
        let read_bytes = match rom.read_to_end(&mut contents) {
            Ok(size) => size,
            Err(e) => {
                panic!("reading rom file: {e}")
            }
        };

        // loading ROM into memory
        // (we start filling memory from location 0x200)
        for i in 0..read_bytes {
            self.memory[i + 0x200] = contents[i];
        }

        // set ROM loaded in memory flag
        self.rom_loaded = true;

        trace!("Chip8::load_rom: exit");
    }

    /// Returns a String that represents the current contents of the CHIP-8 RAM memory
    pub fn dump_memory(&self) -> String {
        trace!("Chip8::dump_memory: start");

        let mut memory_str = String::from("[");
        for i in 0..MAX_MEMORY_SIZE {
            if i == (MAX_MEMORY_SIZE - 1) {
                memory_str += &format!("{:#X}]", self.memory[i]);
            } else {
                memory_str += &format!("{:#X}, ", self.memory[i]);
            }
        }

        trace!("Chip8::dump_memory: exit");

        memory_str
    }

    /// Returns a String that represents the current contents of the CHIP-8 screen.
    /// A CHIP-8 pixel can be white or black, so we have 1 if the pixel is white, 0 otherwise
    pub fn dump_display(&self) -> String {
        trace!("Chip8::dump_display: start");

        // string representation of display
        let mut display_str = String::from("");
        for i in 0..MAX_DISPLAY_SIZE {
            // if i reaches the display width, new line
            if i % DISPLAY_WIDTH == 0 {
                display_str += "\n";
            }
            display_str += &format!("{}", if self.display[i] { 1 } else { 0 });
        }

        trace!("Chip8::dump_display: exit");

        display_str
    }

    /// Returns a String that represents the current contents of the CHIP-8 registers V0-VF
    fn dump_v(&self) -> String {
        trace!("Chip8::dump_v: start");

        let mut v_str = String::from("[");
        for i in 0..V_SIZE {
            if i == (V_SIZE - 1) {
                v_str += &format!("{:#X}]", self.v[i]);
            } else {
                v_str += &format!("{:#X}, ", self.v[i]);
            }
        }

        trace!("Chip8::dump_v: exit");

        v_str
    }

    /// Returns a String that represents the current contents of the CHIP-8 stack
    fn dump_stack(&self) -> String {
        trace!("Chip8::dump_stack: start");

        let mut stack_str = String::from("[");
        for i in 0..MAX_STACK_SIZE {
            if i == (MAX_STACK_SIZE - 1) {
                stack_str += &format!("{:#X}]", self.stack[i]);
            } else {
                stack_str += &format!("{:#X}, ", self.stack[i]);
            }
        }

        trace!("Chip8::dump_stack: exit");

        stack_str
    }

    /// Function that panics on illegal opcode
    fn panic_illegal_opcode(&self) {
        debug!("chip8 state: {}", self);
        panic!("Illegal opcode: `{}`", self.opcode);
    }

    /// Function that panics on illegal opcode with a known category (first nibble)
    ///
    /// # Arguments
    ///
    /// * `category` - The u16 category that is the illegal opcode first nibble
    fn panic_illegal_opcode_category(&self, category: u16) {
        debug!("chip8 state: {}", self);
        panic!(
            "Illegal opcode: `{}` in category `{}`",
            self.opcode, category
        );
    }

    /// Function that emulates one CHIP-8 cycle (one opcode execution):
    /// - fetch, decode, execute opcode
    /// - update timers
    fn emulate_cycle(&mut self) {
        trace!("Chip8::emulate_cycle: start");

        debug!("before fetching: {}", self);

        // fetch the first byte of the opcode
        let first_byte_opcode = self.memory[self.pc as usize];
        debug!("opcode first byte fetch: {:#X}", first_byte_opcode);
        // fetch the second byte of the opcode
        let second_byte_opcode = self.memory[(self.pc + 1) as usize];
        debug!("opcode second byte fetch: {:#X}", second_byte_opcode);
        // combine opcode bytes
        self.opcode = (first_byte_opcode as u16) << 8 | (second_byte_opcode as u16);
        debug!("opcode: {:#X}", self.opcode);

        // CHIP-8 instructions are divided into broad categories by the first nibble (half-byte)
        // so, the first nibble tells us what kind of instruction it is
        let op = self.opcode & 0xF000;
        debug!("first nibble (op): {:#X}", op);

        // second nibble: used to loop up one of the 16 registers (VX) from V0-VF
        let x = self.opcode & 0x0F00;
        debug!("second nibble (x): {:#X}", x);

        // third nibble: used to loop up one of the 16 registers (VY) from V0-VF
        let y = self.opcode & 0x00F0;
        debug!("third nibble (y): {:#X}", y);

        // fourth nibble: 4-bit number
        let n = (self.opcode & 0x000F) as u8;
        debug!("fourth nibble (n): {:#X}", n);

        // second byte (third and fourth nibble). An 8-bit immediate number
        let nn = (self.opcode & 0x00FF) as u8;
        debug!("third and fourth nibble (nn): {:#X}", nn);

        // second, third and fourth nibble. A 12-bit immediate number
        let nnn = self.opcode & 0x0FFF;
        debug!("second, third and fourth nibble (nnn): {:#X}", nnn);

        // match opcode category (first nibble)
        match op {
            // all opcodes with first nibble 0
            0x0000 => {
                match nnn {
                    // clear screen
                    0x00E0 => {
                        debug!("execute: clear screen");
                        // turn off all the pixels
                        for i in 0..MAX_DISPLAY_SIZE {
                            self.display[i] = false;
                        }
                        // redraw screen
                        self.draw = true;

                        // increment PC
                        self.pc += 2;
                    }

                    // return from subroutine
                    0x00EE => {
                        debug!("execute: subroutine return");
                        // pop last address from stack
                        self.sp -= 1;
                        let addr = self.stack[self.sp as usize];
                        // set PC = addr
                        self.pc = addr;
                    }

                    // illegal opcode
                    _ => {
                        self.panic_illegal_opcode_category(op);
                    }
                }
            }

            // opcode with first nibble 1
            // jump to memory location NNN
            0x1000 => {
                debug!("execute: jump");
                // set PC = NNN
                self.pc = nnn;
            }

            // opcode with first nibble 2
            // subroutine call
            0x2000 => {
                debug!("execute: subroutine call");
                // push current PC to stack, so that the subroutine can return later
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                // set PC = NNN
                self.pc = nnn;
            }

            // opcode with first nibble 3
            // skip one instruction if VX == NN
            0x3000 => {
                debug!("execute: skip one instruction if VX == NN");

                if self.v[x as usize] == nn {
                    self.pc += 2
                }

                self.pc += 2
            }

            // opcode with first nibble 4
            // skip one instruction if VX != NN
            0x4000 => {
                debug!("execute: skip one instruction if VX != NN");

                if self.v[x as usize] != nn {
                    self.pc += 2
                }

                self.pc += 2
            }

            // opcodes with first nibble 5
            0x5000 => {
                match n {
                    // opcode with last nibble 0
                    // skip one instruction if VX == VY
                    0x00 => {
                        debug!("execute: skip one instruction if VX == VY");

                        if self.v[x as usize] == self.v[y as usize] {
                            self.pc += 2
                        }

                        self.pc += 2
                    }

                    // illegal opcode
                    _ => {
                        self.panic_illegal_opcode_category(op);
                    }
                }
            }

            // opcode with first nibble 6
            // set VX = NN
            0x6000 => {
                debug!("execute: set VX = NN");

                self.v[x as usize] = nn;

                self.pc += 2
            }

            // opcode with first nibble 7
            // set VX += NN (VF not affected)
            0x7000 => {
                debug!("execute: add: VX += NN (VF not affected)");

                let res = self.v[x as usize] as u16 + nn as u16;

                self.v[x as usize] = res as u8;

                self.pc += 2
            }

            // opcode with first nibble 8
            // logical and arithmetic instructions
            0x8000 => {
                match n {
                    // opcode with last nibble 0
                    // set VX = VY
                    0x00 => {
                        debug!("execute: set VX = VY");

                        self.v[x as usize] = self.v[y as usize];

                        self.pc += 2
                    }

                    // opcode with last nibble 1
                    // set VX |= VY
                    0x01 => {
                        debug!("execute: set VX |= VY");

                        self.v[x as usize] |= self.v[y as usize];

                        self.pc += 2
                    }

                    // opcode with last nibble 2
                    // set VX &= VY
                    0x02 => {
                        debug!("execute: set VX &= VY");

                        self.v[x as usize] &= self.v[y as usize];

                        self.pc += 2
                    }

                    // opcode with last nibble 3
                    // set VX ^= VY
                    0x03 => {
                        debug!("execute: set VX ^= VY");

                        self.v[x as usize] ^= self.v[y as usize];

                        self.pc += 2
                    }

                    // opcode with last nibble 4
                    // set VX += VY (VF affected)
                    0x04 => {
                        debug!("execute: set VX += VY (VF affected)");

                        let res = self.v[x as usize] as u16 + self.v[y as usize] as u16;

                        if res > 255 {
                            // overflow detected
                            // set VF to 1
                            self.v[0xF] = 1;
                        } else {
                            // no overflow detected
                            // set VF to 0
                            self.v[0xF] = 0;
                        }

                        self.v[x as usize] = res as u8;

                        self.pc += 2
                    }

                    // opcode with last nibble 5
                    // set VX = VX - VY (VF affected)
                    0x05 => {
                        debug!("execute: set VX = VX - VY (VF affected)");

                        let a = self.v[x as usize];
                        let b = self.v[y as usize];

                        if a > b {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        self.v[x as usize] = a - b;

                        self.pc += 2
                    }

                    // opcode with last nibble 6
                    // WARN: ambiguous instruction - instruction changed with SUPER-CHIP-8
                    // set VX = VY
                    // set VX >>= 1
                    // set VF to the bit that was shifted out
                    0x06 => {
                        debug!("execute: set VX = VY; VX >>= 1 (VF affected)");

                        self.v[x as usize] = self.v[y as usize];

                        self.v[0xF] = self.v[x as usize] & 0x0F;

                        self.v[x as usize] >>= 1;

                        self.pc += 2
                    }

                    // opcode with last nibble 7
                    // set VX = VY - VX (VF affected)
                    0x07 => {
                        debug!("execute: set VX = VY - VX (VF affected)");

                        let a = self.v[y as usize];
                        let b = self.v[x as usize];

                        if a > b {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        self.v[x as usize] = a - b;

                        self.pc += 2
                    }

                    // opcode with last nibble E
                    // WARN: ambiguous instruction - instruction changed with SUPER-CHIP-8
                    // set VX = VY
                    // set VX <<= 1
                    // set VF to the bit that was shifted out
                    0x0E => {
                        debug!("execute: set VX = VY; VX <<= 1 (VF affected)");

                        self.v[x as usize] = self.v[y as usize];

                        self.v[0xF] = self.v[x as usize] & 0x0F;

                        self.v[x as usize] <<= 1;

                        self.pc += 2
                    }

                    // illegal opcode
                    _ => {
                        self.panic_illegal_opcode_category(op);
                    }
                }
            }

            // opcodes with first nibble 9
            0x9000 => {
                match n {
                    // opcode with last nibble 0
                    // skip one instruction if VX != VY
                    0x00 => {
                        debug!("execute: skip one instruction if VX != VY");

                        if self.v[x as usize] != self.v[y as usize] {
                            self.pc += 2
                        }

                        self.pc += 2
                    }
                    _ => {
                        self.panic_illegal_opcode_category(op);
                    }
                }
            }

            // opcode with first nibble A
            0xA000 => {
                debug!("execute: I = nnn");

                self.i = nnn;

                self.pc += 2
            }

            // illegal opcode
            _ => {
                self.panic_illegal_opcode();
            }
        }

        debug!("after executing: {}", self);

        trace!("Chip8::emulate_cycle: exit");
    }

    /// Function that starts the CHIP-8 emulation
    ///
    /// # Arguments
    ///
    /// * `stepping` - Boolean that enables stepping execution (one cycle at time)
    ///
    /// # Panics
    ///
    /// The function panics if the ROM is not loaded or in case of illegal input during the stepping execution
    pub fn run(&mut self, stepping: bool) {
        if !self.rom_loaded {
            panic!("ROM is not loaded");
        }

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
            self.emulate_cycle();
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

// Display trait implementation for Chip8
impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // string representation of v
        let v_str = self.dump_v();

        // string representation of stack
        let stack_str = self.dump_stack();

        // chip8 string representation: avoided memory and display for excessive length
        write!(f, "Chip8 {{ rom_loaded: {}, current_opcode: {:#X}, memory: [...], V: {}, I: {:#X}, PC: {:#X}, display: [...], draw: {}, stack: {}, SP: {:#X}, timers.delay_timer: {:#X}, timers.sound_timer: {:#X} }}", self.rom_loaded, self.opcode, v_str, self.i, self.pc, self.draw, stack_str, self.sp, self.timers.delay_timer, self.timers.sound_timer)
    }
}
