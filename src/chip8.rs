use crate::memory::Ram;
use cpu::Cpu;

pub struct Chip8 {
    pub ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new()
        }
    }

    pub fn load_cart(&mut self, cart: & Vec<u8>){
        let offset = 0x200;
        for byte in 0..cart.len(){
            self.ram.write_byte((byte + offset) as u16, cart[byte]);
        }
    }

    pub fn load_digits(&mut self, sprites: & [u8; 80]){
        let offset = 0x000;
        for byte in 0..sprites.len(){
            self.ram.write_byte((byte + offset) as u16, sprites[byte])
        }
    }
}