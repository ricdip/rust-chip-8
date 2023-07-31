//! Implementation of CHIP-8 (one cycle emulation)

use crate::chip8::{DISPLAY_HEIGTH, DISPLAY_WIDTH};

use super::{Chip8, MAX_DISPLAY_SIZE};
use rand::Rng;
use tracing::{debug, trace};

impl Chip8 {
    /// Function that emulates one CHIP-8 cycle (one opcode execution):
    /// - fetch, decode, execute opcode
    /// - update timers
    ///
    /// # Arguments
    ///
    /// * `rng` - Mutable reference to a struct that implements the Rng trait used to generate random numbers
    ///
    /// # Panics
    ///
    /// The function panics if the the current opcode is unknown
    pub(super) fn emulate_cycle<R: Rng>(&mut self, rng: &mut R) {
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
        let x = (self.opcode & 0x0F00) >> 8;
        debug!("second nibble (x): {:#X}", x);

        // third nibble: used to loop up one of the 16 registers (VY) from V0-VF
        let y = (self.opcode & 0x00F0) >> 4;
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
            // set I = NNN
            0xA000 => {
                debug!("execute: set I = nnn");

                self.i = nnn;

                self.pc += 2
            }

            // opcode with first nibble B
            // WARN: ambiguous instruction - instruction changed with SUPER-CHIP-8
            // jump with offset
            // set PC = NNN + V0
            0xB000 => {
                debug!("execute: jump with offset: PC = NNN + V0");

                self.pc = nnn + (self.v[0] as u16);
            }

            // opcode with first nibble C
            // random: VX = rand & nn
            0xC000 => {
                debug!("execute: random: VX = rand & nn");

                let rand = rng.gen::<u8>();

                self.v[x as usize] = rand & nn;

                self.pc += 2
            }

            // opcode with first nibble D
            // display
            0xD000 => {
                debug!("execute: display");
                // VX
                let x_coord = self.v[x as usize] % DISPLAY_WIDTH as u8;
                // VY
                let y_coord = self.v[y as usize] % DISPLAY_HEIGTH as u8;
                // H (row)
                let heigth = n;
                // reset register VF
                self.v[0xF] = 0;
                // sprite row data
                let mut sprite_row_data: u8;

                // iterate over sprite rows (max n height)
                for sprite_row in 0..heigth {
                    // break if VY + current_sprite_row is >= 32
                    if (y_coord + sprite_row) >= DISPLAY_HEIGTH as u8 {
                        break;
                    }
                    // get sprite row data from memory starting at location I
                    sprite_row_data = self.memory[self.i as usize + sprite_row as usize];

                    // iterate over 8 bits/pixels of current row
                    for sprite_bit in u8::from(0)..8 {
                        // break if VX + current_sprite_bit is >= 64
                        if (x_coord + sprite_bit) >= DISPLAY_WIDTH as u8 {
                            break;
                        }
                        // retrieve current sprite_row_data bit/pixel
                        // 0x80 = 0x10000000
                        let current_bit = (0x80 >> sprite_bit) & sprite_row_data;
                        // get current (x, y) coords in display
                        let x_y_coord = (x_coord + sprite_bit) as usize
                            + ((y_coord + sprite_row) as usize * DISPLAY_WIDTH);

                        // if current sprite row bit/pixel is set
                        if current_bit != 0 {
                            // if also the pixel in coordinates (x, y) is set
                            if self.display[x_y_coord] {
                                // turn off the pixel
                                self.display[x_y_coord] = false;
                                // set VF = 1
                                self.v[0xF] = 1;
                            } else {
                                // turn on the pixel
                                self.display[x_y_coord] = true;
                            }
                        }
                    }
                }

                // redraw the screen
                self.draw = true;

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
}
