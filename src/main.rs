mod chip8;

fn main() {
    let emu = &mut chip8::Emu::new();
    println!("Initialized emulator with {} bytes of RAM.", emu.memory.len());
    let id_mask: u16 = 0xF000;

    'main: loop {
        let opcode = emu.fetch();
        match opcode & id_mask {
            0x0000 => match opcode {
                0x00E0 => emu.op_00e0_clear_screen(),
                _ => {
                    println!("Unknown opcode: {:04X}", opcode);
                    break 'main;
                }
            },
            0x1000 => {
                let address = opcode & 0x0FFF;
                emu.op_1nnn_jump(address);
            },
            0x6000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                emu.op_6xnn_set_vx(x, nn);
            },
            0x7000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let nn = (opcode & 0x00FF) as u8;
                emu.op_7xnn_add_vx(x, nn);
            },
            0xA000 => {
                let address = opcode & 0x0FFF;
                emu.op_annn_set_i_reg(address);
            },
            0xD000 => {
                let x = ((opcode & 0x0F00) >> 8) as usize;
                let y = ((opcode & 0x00F0) >> 4) as usize;
                let n = (opcode & 0x000F) as u8;
                emu.op_dxyn_draw(x, y, n);
            },
            _ =>{
                println!("Unknown opcode: {:04X}", opcode);
                break 'main;
            }
        }
    }
}
