#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod opcodes {
    use crate::chip8::Chip8;

    impl Chip8 {
        // 0x00E0: Clears the screen (CLS)
        pub fn op_00E0(&mut self) {
            self.video = [0; 2048];
            self.pc += 2;
        }

        // 0x00EE: Returns from subroutine (RET)
        pub fn op_00EE(&mut self) {
            self.pc = self.stack[(self.sp - 1) as usize];
            self.stack[(self.sp - 1) as usize] = 0;

            self.sp -= 1;
            self.pc += 2;
        }
        // 0x1NNN: Jumps to location NNN (JP addr)
        pub fn op_1NNN(&mut self) {
            self.pc = self.opcode & 0x0FFF;
        }

        // 0x2NNN: Calls subroutine at NNN (CALL addr)
        pub fn op_2NNN(&mut self) {
            self.stack[self.sp as usize] = self.opcode;
            self.sp += 1;

            self.pc = self.opcode & 0x0FFF;
        }

        // 0x3XNN: Skips next instruction if Vx = NN (SE Vx, byte)
        pub fn op_3XNN(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;

            // as keyword truncates top bits (0x3XNN -> 0xNN)
            if vx == self.opcode as u8 {
                self.pc += 4;
            } else {
                self.pc += 2;
            }
        }

        // 0x4XNN: Skips next instruction if Vx != NN (SE Vx, byte)
        pub fn op_4XNN(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;

            if vx != self.opcode as u8 {
                self.pc += 4;
            } else {
                self.pc += 2;
            }
        }

        // 0x5XY0: Skips next instruction if Vx = Vy (SE Vx, Vy)
        pub fn op_5XY0(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            if vx == vy {
                self.pc += 4;
            } else {
                self.pc += 2;
            }
        }

        // 0x6XNN: Sets Vx = NN (LD Vx, byte)
        pub fn op_6XNN(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let nn: u8 = self.opcode as u8;

            self.registers[vx as usize] = nn;
            self.pc += 2;
        }

        // 0x7XNN: Sets Vx = Vx + NN (ADD Vx, byte)
        pub fn op_7XNN(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let nn: u8 = self.opcode as u8;

            self.registers[vx as usize] += nn;
            self.pc += 2;
        }

        // 0x8XY0: Sets Vx = Vy (LD Vx, byte)
        pub fn op_8XY0(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] = self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY1: Sets Vx = Vx OR Vy (OR Vx, Vy)
        pub fn op_8XY1(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] |= self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY2: Sets Vx = Vy AND Vy (AND Vx, Vy)
        pub fn op_8XY2(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] &= self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY3: Sets Vx = Vx XOR Vy (XOR Vx, Vy)
        pub fn op_8XY3(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] ^= self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY4: Sets Vx = Vx + Vy (ADD Vx, Vy)
        pub fn op_8XY4(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] += self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY5: Sets Vx = Vx - Vy (SUB Vx, Vy)
        pub fn op_8XY5(&mut self) {
            let vx: u8 = ((self.opcode >> 8) & 0xF) as u8;
            let vy: u8 = ((self.opcode >> 4) & 0xF) as u8;

            self.registers[vx as usize] -= self.registers[vy as usize];
            self.pc += 2;
        }

        // 0x8XY6: Sets Vx = LSB of Vx >> 1
        pub fn op_8XY6() {}

        // 0x8XY7: Sets Vx = Vy - Vx (SUB Vy, Vx)
        pub fn op_8XY7() {}

        // 0x8XYE: Sets Vx = MSB of Vx << 1
        pub fn op_8XYE() {}

        // 0x9XY0: Skips next instruction if Vx != Vy
        pub fn op_9XY0() {}

        // 0xANNN: Sets index = NNN (LD index, addr)
        pub fn op_ANNN() {}

        // 0xBNNN: Jumps to location NNN + V0 (JP V0, addr)
        pub fn op_BNNN() {}

        // 0xCNNN: Sets Vx = random byte AND NN (RND Vx, byte)
        pub fn op_CXNN() {}

        // 0xDXYN: Displays n-byte sprites starting at memory location I for (Vx, Vy)
        pub fn op_DXYN() {}

        // 0xEX9E: Skips next instruction if the key containing V IS pressed (SKP Vx)
        pub fn op_EX9E() {}

        // 0xEXA1: Skips next instruction if the key containing vx is NOT pressed (SKP Vx)
        pub fn op_EXA1() {}

        // 0xFX07: Sets Vx = delay timer value (LD Vx, delay_timer)
        pub fn op_FX07() {}

        // 0xFX0A: Waits for a key press, then stores the value of the key into Vx (LD Vx, K)
        pub fn op_FX0A() {}

        // 0xFX15: Sets delay_timer == Vx (LD display_timer, Vx)
        pub fn op_FX15() {}

        // 0xFX18: Sets sound_timer == Vx (LD sound_timer, Vx)
        pub fn op_FX18() {}

        // 0xFX1E: Sets I = I + Vx (ADD I, Vx)
        pub fn op_FX1E() {}

        // 0xFX29: Sets I = location of Vx's digit sprite (LD F, Vx)
        pub fn op_FX29() {}

        // 0xFX33: Stores BCD (binary coded decimal) representation of Vx in I, I+1, and I+2 (LD B, Vx)
        pub fn op_FX33() {}

        // 0xFX55: Stores registers V0 -> Vx in memory starting at location I (index)
        pub fn op_FX55() {}

        // 0xFX65: Reads registers V0 -> Vx from memory starting at location I (index)
        pub fn op_FX65() {}
    }
}
