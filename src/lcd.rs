use crate::bus::Bus;
use crate::util::{nth_bit,bit_set};
use crate::dma::Dma;
use crate::cpu::Cpu;
use crate::interrupts::InterruptType;

#[derive(Debug, Clone)]
pub struct Lcd {
    pub lcdc: u8, 
    pub lcds: u8,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub ly: u8,
    pub ly_compare: u8,
    pub dma: u8,
    pub bg_palette: u8,
    pub obj_palette: [u8; 2],
    pub win_y: u8,
    pub win_x: u8,

    pub bg_colors: [u32; 4],
    pub sp1_colors: [u32; 4],
    pub sp2_colors: [u32;  4],
}

const DEFAULT_COLRORS: [u32; 4] = [0xFFFFFFFF, 0xFFAAAAAA, 0xFF555555, 0xFF000000];

impl Default for Lcd {
    fn default() -> Self {
        Lcd {
            lcdc: 0x91,
            scroll_x: 0,
            scroll_y: 0,
            ly: 0,
            ly_compare: 0,
            bg_palette: 0xFC,
            obj_palette: [0xFF, 0xFF],
            win_y: 0,
            win_x: 0,
            
            bg_colors: DEFAULT_COLRORS,
            sp1_colors: DEFAULT_COLRORS,
            sp2_colors: DEFAULT_COLRORS,

            dma: 0,
            lcds: (LcdMode::Oam as u8),
        }
    }
}

#[repr(u8)]
pub enum LcdMode {
    Hblank = 0,
    Vblank = 1,
    Oam = 2,
    Xfer = 3,
}

#[repr(u8)]
pub enum StatSrc {
    HBlank = 1 << 3,
    VBlank = 1 << 4,
    Oam = 1 << 5,
    Lyc = 1 << 6,
}

impl Lcd {

    pub fn read(&self, address: u16) -> u8 {
        let offset = (address - 0xFF40) as usize;
        let p = self as *const Lcd as *const u8;

        unsafe { *p.add(offset) }
    }

    pub fn update_palette(&mut self, palette_data: u8, pal: u8) {
        let p_colors = match pal {
            1 => &mut self.sp1_colors,
            2 => &mut self.sp2_colors,
            _ => &mut self.bg_colors,
        };

        p_colors[0] = DEFAULT_COLRORS[(palette_data & 0b11) as usize];
        p_colors[1] = DEFAULT_COLRORS[((palette_data >> 2) & 0b11) as usize];
        p_colors[2] = DEFAULT_COLRORS[((palette_data >> 4) & 0b11) as usize];
        p_colors[3] = DEFAULT_COLRORS[((palette_data >> 6) & 0b11) as usize];
    }

    pub fn write(&mut self, address: u16, value: u8, dma: &mut Dma) {
        let offset = (address - 0xFF40) as usize;
        let p = self as *mut Lcd as *mut u8;

        unsafe { *p.add(offset) = value; }

        if offset == 6 {
            // 0xFF46 = DMA
            dma.start(value);
        }

        match address {
            0xFF47 => self.update_palette(value, 0),
            0xFF48 => self.update_palette(value & 0b11111100, 1),
            0xFF49 => self.update_palette(value & 0b11111100, 2),
            _ => {}
        }
    }

    pub fn get_mode(&self) -> u8 {
        self.lcds & 0b11
    }

    pub fn increment_ly(&mut self, cpu: &mut Cpu) {
        self.ly += 1;

        if self.ly == self.ly_compare {
            self.lcds_lyc_set(true);

            if self.lcds_stat_int(StatSrc::Lyc as u8) {
                cpu.request_interrupt(InterruptType::LcdStat);
            }
        } else {
            self.lcds_lyc_set(false);
        }
    }

    pub fn lcds_stat_int(&self, src: u8) -> bool {
        (self.lcds & src) != 0
    }

    pub fn lcdc_bgw_enable(&self) -> bool {
        nth_bit(self.lcdc as u32, 0)
    }

    pub fn lcdc_obj_enable(&self) -> bool {
        nth_bit(self.lcdc as u32, 1)
    }

    pub fn lcdc_obj_height(&self) -> u8 {
        if nth_bit(self.lcdc as u32, 2) {
            16
        } else {
            8
        }
    }

    pub fn lcdc_bg_map_area(&self) -> u16 {
        if nth_bit(self.lcdc as u32, 3) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn lcdc_bgw_data_area(&self) -> u16 {
        if nth_bit(self.lcdc as u32, 4) {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn lcdc_win_enable(&self) -> bool {
        nth_bit(self.lcdc as u32, 5)
    }

    pub fn lcdc_win_map_area(&self) -> u16 {
        if nth_bit(self.lcdc as u32, 6) {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn lcdc_lcd_enable(&self) -> bool {
        nth_bit(self.lcdc as u32, 7)
    }

    pub fn lcds_mode(&self) -> u8 {
        self.lcds & 0b11
    }

    pub fn lcds_mode_set(&mut self, mode: u8) {
        self.lcds &= !0b11;
        self.lcds |= mode;
    }

    pub fn lcds_lyc(&self) -> bool {
        nth_bit(self.lcds as u32, 2)
    }

    pub fn lcds_lyc_set(&mut self, b: bool) {
        self.lcds = bit_set(self.lcds, 2, b);
    }

}

impl TryFrom<u8> for LcdMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LcdMode::Hblank),
            1 => Ok(LcdMode::Vblank),
            2 => Ok(LcdMode::Oam),
            3 => Ok(LcdMode::Xfer),
            _ => Err(()),
        }
    }
}


