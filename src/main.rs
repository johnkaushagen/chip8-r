const RAM_SIZE: usize = 4096;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

struct Emu {
    memory: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Emu {
    fn op_00e0_clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = false;
        }
    }
}

fn main() {
    let chip8: Emu = Emu {
        memory: [0; RAM_SIZE],
        screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
    };
    println!("Initialized emulator with {} bytes of RAM.", chip8.memory.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_initialization() {
        let chip8 = Emu {
            memory: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
        for &byte in chip8.memory.iter() {
            assert_eq!(byte, 0);
        }
    }

    #[test]
    fn test_op_00e0_clear_screen() {
        let mut chip8 = Emu {
            memory: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        };
        chip8.screen[0] = true;
        chip8.op_00e0_clear_screen();
        for &pixel in chip8.screen.iter() {
            assert_eq!(pixel, false);
        }
    }
}
