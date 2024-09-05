#[derive(Debug, Clone)]
pub struct Ram {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

impl Ram {
    pub const fn new() -> Self {
        Ram {
            wram: [0; 0x2000],
            hram: [0; 0x80],
        }
    }

    pub fn wram_read(&self, address: u16) -> u8 {
        let addr = address.wrapping_sub(0xC000);

        if addr >= 0x2000 {
            panic!("INVALID RAM ADDRESS {:04X}", address);
        }

        self.wram[addr as usize]
    }


    pub fn wram_write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(0xC000);
        self.wram[address as usize] = value;
    }

    pub fn hram_read(&self, address: u16) -> u8 {
        let address = address.wrapping_sub(0xFF80);
        self.hram[address as usize]
    }

    pub fn hram_write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(0xFF80);
        self.hram[address as usize] = value;
    }
}
