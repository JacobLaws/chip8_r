pub struct Chip8 {
    memory   : [u8; 4096],
    registers: [u8; 16],
    stack    : [u16; 16],
    video    : [u8; 2048],
    input    : [u8; 16],

    opcode: u16,
    index : u16,
    pc    : u16,
    sp    : u8,

    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn init_chip8(font_set: [u8; 80]) -> Self {
        Self {
            memory   : Self::init_memory(font_set),
            registers: [0; 16],
            stack    : [0; 16],
            video    : [0; 2048],
            input    : [0; 16],

            opcode: 0,
            index : 0,
            pc    : 512,
            sp    : 0,

            delay_timer: 0,
            sound_timer: 0,
        }
    }

    fn init_memory(font_set: [u8; 80]) -> [u8; 4096] {
        let mut memory: [u8; 4096] = [0; 4096];

        for i in 0..80 {
            memory[0x50 + i] = font_set[i];
        }
        memory
    }
}