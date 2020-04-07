use crate::memory::Ram;
use crate::cpu::Cpu;

pub struct Chip8 {
    pub ram: Ram,
    pub cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
            cpu: Cpu::new()
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

    pub fn read_from_memory(&mut self) -> u16{
        return self.ram.read_byte(self.cpu.I);
    }

    pub fn call_cpu(&mut self, word: u16){
        self.cpu.exec_instr(word);
        self.cpu.I += 1;
    }
}