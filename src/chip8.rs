use crate::memory::Ram;

pub struct Chip8 {
    pub ram: Ram,
    pub I: u16
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            I: 0
        }
    }

    pub fn load_cart(&mut self, cart: & Vec<u8>){
        let offset = 0x200;
        for byte in 0..cart.len(){
            self.ram.write_byte((byte + offset) as u16, cart[byte]);
        }
    }
}