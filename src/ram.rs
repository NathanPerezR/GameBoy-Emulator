#[derive(Debug)]
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

    pub fn wram_read(&mut self, address: u16) -> u8 {
        address.wrapping_sub(0xC000);

        if address >= 0x2000 {
            println!("INVALID RAM ADDRESS");
        }

        return self.wram[address as usize];
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
