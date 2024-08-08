#[derive(Debug)]
struct RamContext {
    wram: [u8; 0x2000],
    hram: [u8; 0x80],
}

impl RamContext {
    pub const fn new() -> Self {
        RamContext {
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


    fn wram_write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(0xC000);
        self.wram[address as usize] = value;
    }

    fn hram_read(&self, address: u16) -> u8 {
        let address = address.wrapping_sub(0xFF80);
        self.hram[address as usize]
    }

    fn hram_write(&mut self, address: u16, value: u8) {
        let address = address.wrapping_sub(0xFF80);
        self.hram[address as usize] = value;
    }
}
