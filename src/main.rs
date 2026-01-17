const RAM_SIZE: usize = 4096;

struct Emu {
    memory: [u8; RAM_SIZE],
}

fn main() {
    let chip8: Emu = Emu {
        memory: [0; RAM_SIZE],
    };
    println!("Initialized emulator with {} bytes of RAM.", chip8.memory.len());
}
