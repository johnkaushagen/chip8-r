const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct Emu {
    pc: u16,
    memory: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Emu {
    fn new() -> Self {
        Emu {
            pc: 0,
            memory: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
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

}

fn main() {
    let chip8 = &mut Emu::new();
    println!("Initialized emulator with {} bytes of RAM.", chip8.memory.len());
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
}
