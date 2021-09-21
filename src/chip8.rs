use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Chip8 {
    register: [u8; 16],
    index_register: u16,
    program_counter: u16,

    stack: [u16; 16],
    stack_pointer: u8,

    memory: [u8; 0x1000],

    pub display: [bool; 64 * 32],

    delay_timer: u8,
    sound_timer: u8,

    keyboard: [bool; 16],

    rng: ThreadRng,
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
            display: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [false; 16],
            rng: rand::thread_rng(),
        }
    }

    fn push(&mut self, value: u16) {
        self.stack[self.stack_pointer as usize] = value;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer as usize]
    }

    fn read_register(&self, register_index: u8) -> u8 {
        self.register[register_index as usize]
    }

    fn write_register(&mut self, register_index: u8, value: u8) {
        self.register[register_index as usize] = value;
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

    fn read_keyboard(&self, keyboard_index: u8) -> bool {
        self.keyboard[keyboard_index as usize]
    }

    fn read_display(&self, i: u8, j: u8) -> bool {
        let i_warped = (i % 64) as usize;
        let j_warped = (j % 32) as usize;
        self.display[64 * j_warped + i_warped]
    }

    fn write_display(&mut self, i: u8, j: u8, value: bool) -> bool {
        let i_warped = (i % 64) as usize;
        let j_warped = (j % 32) as usize;

        // XOR
        let set_value = self.read_display(i, j) ^ value;
        self.display[64 * j_warped + i_warped] = set_value;

        set_value == false
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x200;
    }

    pub fn run(&mut self) {
        self.run_with_callback(|_| {});
    }

    pub fn run_with_callback<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Chip8),
    {
        loop {
            callback(self);

            /*
            info!(
                "V0: {:X}, V1: {:X}, V2: {:X}, V3: {:X}, V4: {:X}, V5: {:X}, V6: {:X}, V7: {:X}, V8: {:X}, V9: {:X}, V10: {:X}, V11: {:X}, V12: {:X}, V13: {:X}, V14: {:X}, V15: {:X}",
                self.register[0],
                self.register[1],
                self.register[2],
                self.register[3],
                self.register[4],
                self.register[5],
                self.register[6],
                self.register[7],
                self.register[8],
                self.register[9],
                self.register[10],
                self.register[11],
                self.register[12],
                self.register[13],
                self.register[14],
                self.register[15]
            );
            */

            // fetch opcode
            let program_index = self.program_counter;
            let opcode = self.read_memory_u16(self.program_counter);
            self.program_counter += 2;

            info!("opcode: {:X}", opcode);

            // process opcode
            match opcode & 0xF000 {
                0x0000 => match opcode {
                    0x00E0 => {
                        // CLS
                        info!("{:X}, CLS", program_index);

                        for v in self.display.iter_mut() {
                            *v = false;
                        }
                    }
                    0x00EE => {
                        // RET
                        info!("{:X}, RET", program_index);

                        self.program_counter = self.pop();
                    }
                    _ => {
                        todo!("SYS");
                    }
                },
                0x1000 => {
                    // JP addr
                    let nnn = opcode & 0x0FFF;
                    info!("{:X}: JP, {:X}", program_index, nnn);

                    self.program_counter = nnn;
                }
                0x2000 => {
                    // CALL addr
                    let nnn = opcode & 0x0FFF;
                    info!("{:X}: CALL, {:X}", program_index, nnn);

                    self.push(self.program_counter);
                    self.program_counter = nnn;
                }
                0x3000 => {
                    // SE Vx, byte
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let kk = (opcode & 0x00FF) as u8;
                    info!("{:X}: SE, V{} {:X}", program_index, x, kk);

                    if self.read_register(x) == kk {
                        self.program_counter += 2;
                    }
                }
                0x4000 => {
                    // SNE Vx, byte
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let kk = (opcode & 0x00FF) as u8;
                    info!("{:X}: SNE, V{} {:X}", program_index, x, kk);

                    if self.read_register(x) != kk {
                        self.program_counter += 2;
                    }
                }
                0x5000 => {
                    match opcode & 0xF00F {
                        0x5000 => {
                            // SE Vx, Vy
                            let x = ((opcode & 0x0F00) >> 8) as u8;
                            let y = ((opcode & 0x00F0) >> 4) as u8;
                            info!("{:X}: SE, V{} V{}", program_index, x, y);

                            if self.read_register(x) == self.read_register(y) {
                                self.program_counter += 2;
                            }
                        }
                        _ => panic!("unknown opcode: {:X}", opcode),
                    }
                }
                0x6000 => {
                    // LD Vx, byte
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let kk = (opcode & 0x00FF) as u8;
                    info!("{:X}: LD, V{}, {:X}", program_index, x, kk);

                    self.write_register(x, kk);
                }
                0x7000 => {
                    // ADD Vx, byte
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let kk = (opcode & 0x00FF) as u8;
                    info!("{:X}: ADD, V{}, {:X}", program_index, x, kk);

                    self.write_register(x, self.read_register(x).wrapping_add(kk));
                }
                0x8000 => match opcode & 0xF00F {
                    0x8000 => {
                        // LD Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: LD, V{}, V{}", program_index, x, y);

                        self.write_register(x, self.read_register(y));
                    }
                    0x8001 => {
                        // OR Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: OR, V{}, V{}", program_index, x, y);

                        self.write_register(x, self.read_register(x) | self.read_register(y));
                    }
                    0x8002 => {
                        // AND Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: AND, V{}, V{}", program_index, x, y);

                        self.write_register(x, self.read_register(x) & self.read_register(y));
                    }
                    0x8003 => {
                        // XOR Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: XOR, V{}, V{}", program_index, x, y);

                        self.write_register(x, self.read_register(x) ^ self.read_register(y));
                    }
                    0x8004 => {
                        // ADD Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: ADD, V{}, V{}", program_index, x, y);

                        self.write_register(
                            x,
                            self.read_register(x).wrapping_add(self.read_register(y)),
                        );
                    }
                    0x8005 => {
                        // SUB Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: SUB, V{}, V{}", program_index, x, y);

                        self.write_register(
                            x,
                            self.read_register(x).wrapping_sub(self.read_register(y)),
                        );
                    }
                    0x8006 => {
                        // SHR Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: SHR, V{}, V{}", program_index, x, y);

                        self.write_register(0xF, x & 0b0000_0001);
                        self.write_register(x, self.read_register(y) >> 1);
                    }
                    0x8007 => {
                        // SUBN Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        let vx = self.read_register(x);
                        let vy = self.read_register(y);
                        info!("{:X}: SUBN, V{}, V{}", program_index, x, y);

                        self.write_register(0xF, if vy > vx { 1 } else { 0 });
                        self.write_register(x, vy.wrapping_sub(vx));
                    }
                    0x800E => {
                        // SHL Vx, Vy
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        info!("{:X}: SHL, V{}, V{}", program_index, x, y);

                        self.write_register(0xF, x & 0b1000_0000);
                        self.write_register(x, self.read_register(y) << 1);
                    }
                    _ => panic!("unknown opcode: {:X}", opcode),
                },
                0x9000 => {
                    match opcode & 0xF00F {
                        0x9000 => {
                            // SNE Vx, Vy
                            let x = ((opcode & 0x0F00) >> 8) as u8;
                            let y = ((opcode & 0x00F0) >> 4) as u8;
                            info!("{:X}: SNE, V{}, V{}", program_index, x, y);

                            if self.read_register(x) != self.read_register(y) {
                                self.program_counter += 2;
                            }
                        }
                        _ => panic!("unknown opcode: {:X}", opcode),
                    }
                }
                0xA000 => {
                    // LD I, addr
                    let nnn = opcode & 0x0FFF;
                    info!("{:X}: LD, I, {:X}", program_index, nnn);

                    self.index_register = nnn;
                }
                0xB000 => {
                    // JP V0, addr
                    let nnn = opcode & 0x0FFF;
                    info!("{:X}: JP, V0, {:X}", program_index, nnn);

                    self.program_counter = (self.read_register(0) as u16) + nnn;
                }
                0xC000 => {
                    // RND Vx, byte
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let kk = (opcode & 0x00FF) as u8;
                    info!("{:X}: RND, V{}, {:X}", program_index, x, kk);

                    let rnd: u8 = self.rng.gen_range(0..255);
                    self.write_register(x, rnd & kk);
                }
                0xD000 => {
                    // DRW Vx, Vy, nibble
                    let x = ((opcode & 0x0F00) >> 8) as u8;
                    let y = ((opcode & 0x00F0) >> 4) as u8;
                    let n = (opcode & 0x000F) as u8;
                    info!("{:X}: DRW, V{}, V{}, {:b}", program_index, x, y, n);

                    let vx = self.read_register(x);
                    let vy = self.read_register(y);

                    // draw
                    let mut erased = false;
                    for i in 0..n {
                        let v = self.read_memory(self.index_register + (i as u16));
                        erased |= self.write_display(vx, vy + i, (v & 0b1000_0000) != 0);
                        erased |= self.write_display(vx + 1, vy + i, (v & 0b0100_0000) != 0);
                        erased |= self.write_display(vx + 2, vy + i, (v & 0b0010_0000) != 0);
                        erased |= self.write_display(vx + 3, vy + i, (v & 0b0001_0000) != 0);
                        erased |= self.write_display(vx + 4, vy + i, (v & 0b0000_1000) != 0);
                        erased |= self.write_display(vx + 5, vy + i, (v & 0b0000_0100) != 0);
                        erased |= self.write_display(vx + 6, vy + i, (v & 0b0000_0010) != 0);
                        erased |= self.write_display(vx + 7, vy + i, (v & 0b0000_0001) != 0);
                    }

                    // set VF
                    self.write_register(0xF, if erased { 1 } else { 0 });
                }
                0xE000 => match opcode & 0xF0FF {
                    0xE09E => {
                        // SKP Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: SKP, V{}", program_index, x);

                        if self.read_keyboard(x) {
                            self.program_counter += 2;
                        }
                    }
                    0xE0A1 => {
                        // SKNP Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: SKNP, V{}", program_index, x);

                        if !self.read_keyboard(x) {
                            self.program_counter += 2;
                        }
                    }
                    _ => panic!("unknown opcode: {:X}", opcode),
                },
                0xF000 => match opcode & 0xF0FF {
                    0xF007 => {
                        // LD Vx, DT
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: LD, V{}, DT", program_index, x);

                        self.write_register(x, self.delay_timer);
                    }
                    0xF00A => {
                        // LD Vx, K
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: LD, V{}, K", program_index, x);

                        // wait until any key pressed
                        let mut key_index = 0;
                        loop {
                            let mut key_pressed = false;
                            for (index, value) in self.keyboard.iter().enumerate() {
                                if *value {
                                    key_index = index;
                                    key_pressed = true;
                                }
                            }

                            if key_pressed {
                                break;
                            }
                        }

                        self.write_register(x, self.keyboard[key_index] as u8);
                    }
                    0xF015 => {
                        // LD DT, Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: LD, DT, V{}", program_index, x);

                        self.delay_timer = self.read_register(x);
                    }
                    0xF018 => {
                        // LD ST, Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: LD, ST, V{}", program_index, x);

                        self.sound_timer = self.read_register(x);
                    }
                    0xF01E => {
                        // ADD I, Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!(
                            "{:X}: ADD, {:X}, V{}",
                            program_index, self.index_register, x
                        );

                        self.index_register += self.read_register(x) as u16;
                    }
                    0xF029 => {
                        todo!("LD F, Vx")
                    }
                    0xF033 => {
                        // LD B, Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!("{:X}: LD, B, V{}", program_index, x);

                        let vx = self.read_register(x);
                        let hundred = (vx / 100) % 10;
                        let ten = (vx / 10) % 10;
                        let one = vx % 10;

                        self.write_memory(self.index_register, hundred);
                        self.write_memory(self.index_register + 1, ten);
                        self.write_memory(self.index_register + 2, one);
                    }
                    0xF055 => {
                        // LD [I], Vx
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!(
                            "{:X}: LD, [{:X}], V{}",
                            program_index, self.index_register, x
                        );

                        for i in 0..x {
                            self.write_memory(
                                self.index_register + (i as u16),
                                self.read_register(i),
                            );
                        }
                    }
                    0xF065 => {
                        // LD Vx, [I]
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        info!(
                            "{:X}: LD, V{}, [{:X}]",
                            program_index, x, self.index_register
                        );

                        for i in 0..x {
                            self.write_register(
                                i,
                                self.read_memory(self.index_register + (i as u16)),
                            );
                        }
                    }
                    _ => panic!("unknown opcode: {:X}", opcode),
                },
                _ => panic!("unknown opcode: {:X}", opcode),
            }
        }
    }
}
