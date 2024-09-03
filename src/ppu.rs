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
            let bytes = unsafe {
                std::slice::from_raw_parts_mut(self.oam_ram.as_mut_ptr() as *mut u8, 40 * std::mem::size_of::<OamEntry>())
            };
            bytes[index] = value;
        }
    }

    pub fn oam_read(&self, address: u16) -> u8 {
        if address >= 0xFE00 {
            let index = (address - 0xFE00) as usize;
            let bytes = unsafe {
                std::slice::from_raw_parts(self.oam_ram.as_ptr() as *const u8, 40 * std::mem::size_of::<OamEntry>())
            };
            bytes[index]
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

    pub fn set_f_cgb_pn(&mut self, value: u8) {
        self.flags = (self.flags & 0xF8) | (value & 0x07);
    }

    pub fn f_cgb_pn(&self) -> u8 {
        self.flags & 0x07
    }

    pub fn set_f_x_flip(&mut self, value: bool) {
        if value {
            self.flags |= 0x20;
        } else {
            self.flags &= !0x20;
        }
    }

    pub fn f_x_flip(&self) -> bool {
        self.flags & 0x20 != 0
    }
}
