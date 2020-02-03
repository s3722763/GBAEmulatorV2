/*
pub struct Memory {
    memory: Vec<u8>
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            //TODO: Make flexable
            memory: vec![0;16777216]
        }
    }
}

impl Memory {
    pub fn read_word(&self, address: usize) -> u32 {
        let high = self.read_half_word(address) as u32;
        let low = self.read_half_word(address + 2) as u32;
        return low | (high << 16);
    }

    pub fn read_half_word(&self, address: usize) -> u16 {
        let high = self.read_byte(address) as u16;
        let low = self.read_byte(address + 1) as u16;
        return low | (high << 8);
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn write_word(&mut self, address: usize, data: u32) {
        //TODO: find out why this and
        let address = address & 0xFFFFFFFC;
        self.write_half_word(address, data as u16);
        self.write_half_word(address + 2, (data >> 16) as u16);
    }

    pub fn write_half_word(&mut self, address: usize, data: u16) {
        self.write_byte(address, data as u8);
        self.write_byte(address + 1, (data >> 8) as u8);
    }

    pub fn write_byte(&mut self, address: usize, data: u8) {
        self.memory[address] = data;
    }
}*/
