mod chip8;
mod opcode;

use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut f = File::open("test_opcode.ch8").expect("Failed to open the file");

    let mut program: Vec<u8> = Vec::new();
    f.read_to_end(&mut program)
        .expect("failed to read the file");
}
