pub struct Chip8 {
    register: [u8; 16],
    index_register: u16,
    program_counter: u16,

    stack: [u16; 16],
    stack_pointer: u16,

    memory: [u8; 0x1000],

    display: [u8; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    key: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            register: [0; 16],
            index_register: 0,
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
            memory: [0; 0x1000],
            display: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            key: [0; 16],
        }
    }

    fn read_memory(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn read_memory_u16(&self, address: u16) -> u16 {
        let hi = self.read_memory(address) as u16;
        let low = self.read_memory(address + 1) as u16;
        (hi << 8) | low
    }

    fn write_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn write_memory_u16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let low = (value & 0xff) as u8;
        self.write_memory(address, hi);
        self.write_memory(address + 1, low);
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x200;
    }

    pub fn run(&mut self) {
        loop {
            // fetch opcode
            let opcode = self.read_memory_u16(self.program_counter);
            self.program_counter += 2;

            // process opcode
            match opcode & 0xF000 {
                0x0000 => match opcode {
                    0x00E0 => {
                        todo!("CLS");
                    }
                    0x00EE => {
                        todo!("RET");
                    }
                    _ => {
                        todo!("SYS");
                    }
                },
                0x1000 => {
                    todo!("JP");
                }
                0x2000 => {
                    todo!("CALL");
                }
                0x3000 => {
                    todo!("SE");
                }
                0x4000 => {
                    todo!("SNE");
                }
                0x5000 => {
                    todo!("SE");
                }
                0x6000 => {
                    todo!("LD");
                }
                0x7000 => {
                    todo!("ADD");
                }
                0x8000 => match opcode & 0xF00F {
                    0x8000 => {
                        todo!("LD");
                    }
                    0x8001 => {
                        todo!("OR");
                    }
                    0x8002 => {
                        todo!("AND");
                    }
                    0x8003 => {
                        todo!("XOR");
                    }
                    0x8004 => {
                        todo!("ADD");
                    }
                    0x8005 => {
                        todo!("SUB");
                    }
                    0x8006 => {
                        todo!("SHR");
                    }
                    0x8007 => {
                        todo!("SUBN");
                    }
                    0x800E => {
                        todo!("SHL");
                    }
                    _ => panic!("unknown opcode: {:x}", opcode),
                },
                0x9000 => {
                    todo!("SNE");
                }
                0xA000 => {
                    todo!("LD");
                }
                0xB000 => {
                    todo!("JP");
                }
                0xC000 => {
                    todo!("RND");
                }
                0xD000 => {
                    todo!("DRW");
                }
                0xE000 => match opcode & 0xF0FF {
                    0xE09E => {
                        todo!("SKP");
                    }
                    0xE0A1 => {
                        todo!("SKNP");
                    }
                    _ => panic!("unknown opcode: {:x}", opcode),
                },
                0xF000 => match opcode & 0xF0FF {
                    0xF007 => {
                        todo!("LD");
                    }
                    0xF00A => {
                        todo!("LD");
                    }
                    0xF015 => {
                        todo!("LD");
                    }
                    0xF018 => {
                        todo!("LD");
                    }
                    0xF01E => {
                        todo!("ADD");
                    }
                    0xF029 => {
                        todo!("LD");
                    }
                    0xF033 => {
                        todo!("LD");
                    }
                    0xF055 => {
                        todo!("LD");
                    }
                    0xF065 => {
                        todo!("LD");
                    }
                    _ => panic!("unknown opcode: {:x}", opcode),
                },
                _ => panic!("unknown opcode: {:x}", opcode),
            }
        }
    }
}
