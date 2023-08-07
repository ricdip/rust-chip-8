# rust-chip-8

A Rust implementation of CHIP-8 written for fun and learning purposes.

## VM description

- **opcodes**: 35, all 16-bit long 

- **RAM memory**: 4096 memory locations, all of which are 8-bit long

- **V0-VF**: 15 general purpose CPU registers, all of which are 8-bit long. VF is used for "carry flag"

- **I**: 1 Index Register 16-bit long. Points at locations in memory

- **PC**: 1 Program Counter 16-bit long. Points at current instruction in memory

- **display**: black and white graphics, total of 2048 pixels (resolution: 64 x 32)

- **stack**: stores 16-bit addresses and has 16 levels. Used to remember the current location before jump is performed

- **SP**: 1 Stack Pointer 8-bit long. Used to remember which level of the stack is currently used

- **delay timer**: 8-bit timer register that count at 60Hz, is decremented at a rate of 60Hz until it reaches 0

- **sound timer**: 8-bit timer register that count at 60Hz, is decremented at a rate of 60Hz until it reaches 0. It gives off a beeping sound when its value is non-zero

## Used ROMs

- [IBM logo](https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8)

## Help message

```
A Rust implementation of CHIP-8 written for fun and learning purposes

Usage: rust-chip-8 [OPTIONS] --rom-file <FILE>

Options:
  -f, --rom-file <FILE>     Path to CHIP-8 ROM file to run
  -q, --quiet               Enable quiet logging
  -d, --debug               Enable debug logging
  -t, --trace               Enable trace logging
  -s, --stepping            Enable one step at time execution
  -r, --random-seed <SEED>  Random seed [default: 10]
  -h, --help                Print help
  -V, --version             Print version
```

## Build debug version (in target directory)

```bash
user@host:~$ cargo build
```

## Install (in $HOME directory)

```bash
user@host:~$ cargo install --path .
```

## Uninstall 

```bash
user@host:~$ cargo uninstall
```

## Run

```bash
user@host:~$ rust-chip-8 -f roms/IBM_logo.ch8
```

## Docs build and open

```bash
user@host:~$ cargo doc --open
```

## References

Some really helpful references that I used:

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator)

- [CHIP-8](https://www.wikiwand.com/en/CHIP-8)
