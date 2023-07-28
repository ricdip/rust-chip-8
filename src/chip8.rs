//! Implementation of CHIP-8

use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

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
    pub fn reset(&mut self) {
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
    }

    /// Loads CHIP-8 fontset into memory at locations 0x00-0x50
    fn load_fontset(&mut self) {
        // load fontset into memory (0x00-0x50)
        for i in 0x00..0x50 {
            self.memory[i] = CHIP8_FONTSET[i];
        }
    }

    /// Returns a new CHIP-8 instance ready to load a new ROM file
    pub fn new() -> Self {
        // create new chip8 instance
        let mut chip8 = Self {
            rom_loaded: false,
            opcode: 0,
            memory: [0; MAX_MEMORY_SIZE],
            v: [0; V_SIZE],
            i: 0,
            pc: 0x200,
            display: [false; MAX_DISPLAY_SIZE],
            stack: [0; MAX_STACK_SIZE],
            sp: 0,
            timers: Timers {
                delay_timer: 0,
                sound_timer: 0,
            },
        };
        // load fontset
        chip8.load_fontset();
        // return created instance
        chip8
    }

    /// Loads a ROM file into the memory of the current CHIP-8 instance
    ///
    /// # Arguments
    ///
    /// * `file` - The PathBuf that holds the path to the ROM file
    ///
    /// # Panics
    ///
    /// The function panics in case of errors during opening and reading of the ROM file
    pub fn load_rom(&mut self, file: PathBuf) {
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
    }

    /// Returns the current contents of CHIP-8 RAM memory
    pub fn dump_memory(&self) -> [u8; MAX_MEMORY_SIZE] {
        self.memory
    }

    /// Returns a String that represents the current contents of the CHIP-8 screen.
    /// A CHIP-8 pixel can be white or black, so we have 1 if the pixel is white, 0 otherwise
    pub fn dump_display(&self) -> String {
        // string representation of display
        let mut display_str = String::from("");
        for i in 0..MAX_DISPLAY_SIZE {
            // if i reaches the display width, new line
            if i % DISPLAY_WIDTH == 0 {
                display_str += "\n";
            }
            display_str += &format!("{}", if self.display[i] { 1 } else { 0 });
        }
        display_str
    }
}

// Display trait implementation for Chip8
impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // string representation of v
        let mut v_str = String::from("[");
        for i in 0..V_SIZE {
            if i == (V_SIZE - 1) {
                v_str += &format!("{:#X}]", self.v[i]);
            } else {
                v_str += &format!("{:#X}, ", self.v[i]);
            }
        }

        // string representation of stack
        let mut stack_str = String::from("[");
        for i in 0..MAX_STACK_SIZE {
            if i == (MAX_STACK_SIZE - 1) {
                stack_str += &format!("{:#X}]", self.stack[i]);
            } else {
                stack_str += &format!("{:#X}, ", self.stack[i]);
            }
        }

        // chip8 string representation: avoided memory and display for excessive length
        write!(f, "Chip8 {{ rom_loaded: {}, current_opcode: {:#X}, memory: [...], V: {}, I: {:#X}, PC: {:#X}, display: [...], stack: {}, SP: {:#X}, timers.delay_timer: {:#X}, timers.sound_timer: {:#X} }}", self.rom_loaded, self.opcode, v_str, self.i, self.pc, stack_str, self.sp, self.timers.delay_timer, self.timers.sound_timer)
    }
}
