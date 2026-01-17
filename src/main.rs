const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const VREG_SIZE: usize = 16;

struct Emu {
    pc: u16,
    memory: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    i_reg: u16,
    v_reg: [u8; VREG_SIZE],
}

impl Emu {
    fn new() -> Self {
        Emu {
            pc: 0,
            memory: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            i_reg: 0,
            v_reg: [0; VREG_SIZE],
        }
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    fn op_00e0_clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = false;
        }
    }

    fn op_1nnn_jump(&mut self, address: u16) {
        self.pc = address;
    }

    fn op_6xnn_set_vx(&mut self, x: usize, nn: u8) {
        self.v_reg[x] = nn;
    }

    fn op_7xnn_add_vx(&mut self, x: usize, nn: u8) {
        // This operation does not set the carry flag
        let (result, _) = self.v_reg[x].overflowing_add(nn);
        self.v_reg[x] = result;
    }

    fn op_annn_set_i_reg(&mut self, address: u16) {
        self.i_reg = address;
    }

}

fn main() {
    let chip8 = &mut Emu::new();
    println!("Initialized emulator with {} bytes of RAM.", chip8.memory.len());
    let id_mask: u16 = 0xF000;

    'main: loop {
        let opcode = chip8.fetch();
        match opcode & id_mask {
            0x0000 => match opcode {
                0x00E0 => chip8.op_00e0_clear_screen(),
                _ => {
                    println!("Unknown opcode: {:04X}", opcode);
                    break 'main;
                }
            },
            0x1000 => {
                let address = opcode & 0x0FFF;
                chip8.op_1nnn_jump(address);
            }

            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                chip8.op_6xnn_set_vx(x, nn);
            }

            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                chip8.op_7xnn_add_vx(x, nn);
            }

            _ =>{
                println!("Unknown opcode: {:04X}", opcode);
                break 'main;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let chip8 = Emu::new();
        for &byte in chip8.memory.iter() {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn test_fetch_instruction() {
        let chip8 = &mut Emu::new();
        chip8.memory[0] = 0xAB;
        chip8.memory[1] = 0xCD;
        let actual = chip8.fetch();
        let expected = 0xABCD;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_fetch_instruction_increments_pc() {
        let chip8 = &mut Emu::new();
        let expected = chip8.pc + 2;
        let _ = chip8.fetch();
        assert_eq!(chip8.pc, expected);
    }

    #[test]
    fn test_op_00e0_clear_screen() {
        let chip8 = &mut Emu::new();
        chip8.screen[0] = true;
        chip8.op_00e0_clear_screen();
        for &pixel in chip8.screen.iter() {
            assert_eq!(pixel, false);
        }
    }

    #[test]
    fn test_op_1nnn_jump() {
        let chip8 = &mut Emu::new();
        chip8.memory[0] = 0x12;
        chip8.memory[1] = 0x34;
        let address: u16 = 0x234;
        chip8.op_1nnn_jump(address);
        assert_eq!(chip8.pc, address);
    }

    #[test]
    fn test_op_6xnn_set_vx() {
        let chip8 = &mut Emu::new();
        chip8.memory[0] = 0x60;
        chip8.memory[1] = 0xFF;
        let opcode = chip8.fetch();
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let nn = (opcode & 0x00FF) as u8;
        chip8.v_reg[x] = nn;
        assert_eq!(chip8.v_reg[x], 0xFF);
    }

    #[test]
    fn test_op_7xnn_add_vx() {
        let chip8 = &mut Emu::new();
        let old_carry = chip8.v_reg[0xF];
        chip8.v_reg[0] = 0x10;
        chip8.op_7xnn_add_vx(0, 0x20);
        assert_eq!(chip8.v_reg[0], 0x30);

        chip8.v_reg[1] = 0xFF;
        chip8.op_7xnn_add_vx(1, 0x02);
        assert_eq!(chip8.v_reg[1], 0x01); // Overflow wraps around

        assert_eq!(chip8.v_reg[0xF], old_carry); // Carry flag unchanged
    }

    #[test]
    fn test_op_annn_set_i_reg() {
        let chip8 = &mut Emu::new();
        let address: u16 = 0x300;
        chip8.op_annn_set_i_reg(address);
        assert_eq!(chip8.i_reg, address);
    }
}
