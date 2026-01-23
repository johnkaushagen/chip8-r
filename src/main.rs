mod chip8;

fn main() {
    let emu = &mut chip8::Emu::new();
    println!("Initialized emulator with {} bytes of RAM.", emu.memory.len());
    let mut cycles: usize = 0;
    let max_cycles: usize = 10_000;
    'main: loop {
        emu.cycle();
        cycles += 1;
        if cycles > max_cycles {
            break 'main;
        }
    }
}
