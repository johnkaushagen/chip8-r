const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const VREG_SIZE: usize = 16;

pub struct Emu {
    pc: u16,
    pub memory: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    i_reg: u16,
    v_reg: [u8; VREG_SIZE],
}

impl Emu {
    pub fn new() -> Self {
        Emu {
            pc: 0,
            memory: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            i_reg: 0,
            v_reg: [0; VREG_SIZE],
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    pub fn op_00e0_clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = false;
        }
    }

    pub fn op_1nnn_jump(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn op_6xnn_set_vx(&mut self, x: usize, nn: u8) {
        self.v_reg[x] = nn;
    }

    pub fn op_7xnn_add_vx(&mut self, x: usize, nn: u8) {
        // This operation does not set the carry flag
        let (result, _) = self.v_reg[x].overflowing_add(nn);
        self.v_reg[x] = result;
    }

    pub fn op_annn_set_i_reg(&mut self, address: u16) {
        self.i_reg = address;
    }

    pub fn op_dxyn_draw(&mut self, x: usize, y: usize, n: u8) {
        self.v_reg[0xF] = 0;
        let left = self.v_reg[x] as usize % SCREEN_WIDTH;
        let top = self.v_reg[y] as usize % SCREEN_HEIGHT;
        let mut flipped = false;
        for row in 0..n as usize {
            for col in 0..8 as usize {
                let sprite_data = self.memory[(self.i_reg + row as u16) as usize];
                let pixel = ((sprite_data >> (7 - col)) & 0x1) == 1;
                let screen_x = left + col; // Don't wrap
                let screen_y = top + row;
                if screen_x > SCREEN_WIDTH || screen_y > SCREEN_HEIGHT {
                    continue;
                }
                let index = screen_y * SCREEN_WIDTH + screen_x;
                if pixel && self.screen[index] {
                    flipped = true;
                }
                self.screen[index] ^= pixel;
                if flipped { self.v_reg[0xF] = 1; }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let emu = Emu::new();
        for &byte in emu.memory.iter() {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn test_fetch_instruction() {
        let emu = &mut Emu::new();
        emu.memory[0] = 0xAB;
        emu.memory[1] = 0xCD;
        let actual = emu.fetch();
        let expected = 0xABCD;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fetch_instruction_increments_pc() {
        let emu = &mut Emu::new();
        let expected = emu.pc + 2;
        let _ = emu.fetch();
        assert_eq!(emu.pc, expected);
    }

    #[test]
    fn test_op_00e0_clear_screen() {
        let emu = &mut Emu::new();
        emu.screen[0] = true;
        emu.op_00e0_clear_screen();
        for &pixel in emu.screen.iter() {
            assert_eq!(pixel, false);
        }
    }

    #[test]
    fn test_op_1nnn_jump() {
        let emu = &mut Emu::new();
        emu.memory[0] = 0x12;
        emu.memory[1] = 0x34;
        let address: u16 = 0x234;
        emu.op_1nnn_jump(address);
        assert_eq!(emu.pc, address);
    }

    #[test]
    fn test_op_6xnn_set_vx() {
        let emu = &mut Emu::new();
        emu.memory[0] = 0x60;
        emu.memory[1] = 0xFF;
        let opcode = emu.fetch();
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        emu.v_reg[x] = nn;
        assert_eq!(emu.v_reg[x], 0xFF);
    }

    #[test]
    fn test_op_7xnn_add_vx() {
        let emu = &mut Emu::new();
        let old_carry = emu.v_reg[0xF];
        emu.v_reg[0] = 0x10;
        emu.op_7xnn_add_vx(0, 0x20);
        assert_eq!(emu.v_reg[0], 0x30);

        emu.v_reg[1] = 0xFF;
        emu.op_7xnn_add_vx(1, 0x02);
        assert_eq!(emu.v_reg[1], 0x01); // Overflow wraps around

        assert_eq!(emu.v_reg[0xF], old_carry); // Carry flag unchanged
    }

    #[test]
    fn test_op_annn_set_i_reg() {
        let emu = &mut Emu::new();
        let address: u16 = 0x300;
        emu.op_annn_set_i_reg(address);
        assert_eq!(emu.i_reg, address);
    }

    #[test]
    fn test_dxyn_draw() {
        let emu = &mut Emu::new();
        emu.i_reg = 0x200;
        emu.memory[0x200] = 0b11110000;
        emu.v_reg[0] = 0; // x coordinate
        emu.v_reg[1] = 0; // y coordinate
        emu.op_dxyn_draw(0, 1, 1);
        assert_eq!(emu.screen[0], true);
        assert_eq!(emu.screen[1], true);
        assert_eq!(emu.screen[2], true);
        assert_eq!(emu.screen[3], true);
        assert_eq!(emu.screen[4], false);
        assert_eq!(emu.v_reg[0xF], 0); // No pixels flipped

        // Draw the same sprite again to test pixel flipping
        emu.op_dxyn_draw(0, 1, 1);
        assert_eq!(emu.screen[0], false);
        assert_eq!(emu.screen[1], false);
        assert_eq!(emu.screen[2], false);
        assert_eq!(emu.screen[3], false);
        assert_eq!(emu.v_reg[0xF], 1); // Pixels flipped

        emu.memory[0x201] = 0b11111111;
        emu.v_reg[0] = 60; // x coordinate near edge
        emu.v_reg[1] = 30; // y coordinate near edge
        emu.op_dxyn_draw(0, 1, 1);
        // Only the first 4 pixels should be drawn
        assert_eq!(emu.screen[30 * SCREEN_WIDTH + 60], true);
        assert_eq!(emu.screen[30 * SCREEN_WIDTH + 61], true);
        assert_eq!(emu.screen[30 * SCREEN_WIDTH + 62], true);
        assert_eq!(emu.screen[30 * SCREEN_WIDTH + 63], true);
        assert_eq!(emu.v_reg[0xF], 0); // No pixels flipped
    }
}
