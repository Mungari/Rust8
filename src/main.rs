use std::fs::File;
use std::io::Read;
use chip8::Chip8;

mod memory;
mod chip8;

fn main() {
    let mut file = File::open("roms/TANK").unwrap();
    let mut rom = Vec::<u8>::new();
    file.read_to_end(&mut rom).unwrap();
    let mut chip8 = Chip8::new();
    chip8.load_cart(&rom);

    print!("{:#X?}", &chip8.ram.mem[0x200 as usize .. rom.len() as usize])
}
