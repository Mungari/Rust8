pub struct Ram {
    pub mem: [u8; 4096]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            mem: [0; 4096]
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8{
        return self.mem[address as usize];
    }

    pub fn write_byte(&mut self, address: u16, word: u8){
        self.mem[address as usize] = word;
    }
}