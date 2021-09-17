use std::collections::HashMap;

struct OpCode {
    opcode: u16,
    param_mask: u16,
}

impl OpCode {
    fn new(opcode: u16, param_mask: u16) -> OpCode {
        OpCode { opcode, param_mask }
    }
}

lazy_static! {
    static ref OP_CODES: Vec<OpCode> = vec![
        OpCode::new(0x0000, 0x0FFF),
        OpCode::new(0x00E0, 0x0000),
        OpCode::new(0x1000, 0x0FFF),
        OpCode::new(0x2000, 0x0FFF),
        OpCode::new(0x3000, 0x0FFF),
        OpCode::new(0x4000, 0x0FFF),
        OpCode::new(0x5000, 0x0FF0),
        OpCode::new(0x6000, 0x0FFF),
        OpCode::new(0x7000, 0x0FFF),
        OpCode::new(0x8000, 0x0FF0),
        OpCode::new(0x8001, 0x0FF0),
        OpCode::new(0x8002, 0x0FF0),
        OpCode::new(0x8003, 0x0FF0),
        OpCode::new(0x8004, 0x0FF0),
        OpCode::new(0x8005, 0x0FF0),
        OpCode::new(0x8006, 0x0FF0),
        OpCode::new(0x8007, 0x0FF0),
        OpCode::new(0x800E, 0x0FF0),
        OpCode::new(0x9000, 0x0FF0),
        OpCode::new(0xA000, 0x0FFF),
        OpCode::new(0xB000, 0x0FFF),
        OpCode::new(0xC000, 0x0FFF),
        OpCode::new(0xD000, 0x0FFF),
        OpCode::new(0xE09E, 0x0F00),
        OpCode::new(0xE0A1, 0x0F00),
        OpCode::new(0xF007, 0x0F00),
        OpCode::new(0xF00A, 0x0F00),
        OpCode::new(0xF015, 0x0F00),
        OpCode::new(0xF018, 0x0F00),
        OpCode::new(0xF01E, 0x0F00),
        OpCode::new(0xF029, 0x0F00),
        OpCode::new(0xF033, 0x0F00),
        OpCode::new(0xF055, 0x0F00),
        OpCode::new(0xF065, 0x0F00),
    ];
    static ref OP_CODES_HASHMAP: HashMap<u16, &'static OpCode> = {
        let mut map = HashMap::new();
        for op in &*OP_CODES {
            map.insert(op.opcode, op);
        }
        map
    };
}
