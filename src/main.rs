struct Chip8 {
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

    fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x200;
    }

    fn run(&mut self) {
        loop {
            // fetch opcode
            let opcode = self.read_memory_u16(self.program_counter);
            self.program_counter += 2;

            // process opcode
        }
    }
}

fn main() {
    println!("Hello, world!");
}
