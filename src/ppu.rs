#[derive(Clone, Copy)]
pub struct Ppu {
    oam_ram: [OamEntry; 40],
    vram: [u8; 0x2000],
}

impl Ppu {

    pub fn new() -> Self {
        Ppu {
            oam_ram: [OamEntry::new(0,0,0,0); 40],
            vram: [0; 0x2000],
        }
    }

    pub fn oam_write(&mut self, address: u16, value: u8) {
        if address >= 0xFE00 {
            let index = (address - 0xFE00) as usize;
            let entry_index = index / 4;
            let offset = index % 4;

            match offset {
                0 => self.oam_ram[entry_index].y = value,
                1 => self.oam_ram[entry_index].x = value,
                2 => self.oam_ram[entry_index].tile = value,
                3 => self.oam_ram[entry_index].flags = value,
                _ => unreachable!(),
            }
        }
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        if address >= 0xFE00 {
            let index = (address - 0xFE00) as usize;
            let entry_index = index / 4;
            let offset = index % 4;

            match offset {
                0 => self.oam_ram[entry_index].y,
                1 => self.oam_ram[entry_index].x,
                2 => self.oam_ram[entry_index].tile,
                3 => self.oam_ram[entry_index].flags,
                _ => unreachable!(),
            }
        } else {
            0 
        }
    }

    pub fn vram_write(&mut self, address: u16, value: u8) {
        self.vram[(address - 0x8000) as usize] = value;
    }

    pub fn vram_read(&self, address: u16) -> u8 {
        self.vram[(address - 0x8000) as usize]
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OamEntry {
    pub y: u8,
    pub x: u8,
    pub tile: u8,
    pub flags: u8,
}

impl OamEntry {
    pub fn new(y: u8, x: u8, tile: u8, flags: u8) -> Self {
        OamEntry { y, x, tile, flags }
    }
}
