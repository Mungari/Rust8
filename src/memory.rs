pub struct Ram {
    pub mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            mem: [0; 4096]
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u16{
        return u16::from_le_bytes([self.mem[address as usize], self.mem[address as usize + 1]])
    }

    pub fn write_byte(&mut self, address: u16, word: u8){
        self.mem[address as usize] = word;
    }
}