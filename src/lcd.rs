use crate::bus::Bus;
use crate::util::{nth_bit,bit_set};

#[derive(Debug)]
pub struct Lcd {
    lcdc: u8, 
    lcds: u8,
    scroll_y: u8,
    scroll_x: u8,
    ly: u8,
    ly_compare: u8,
    dma: u8,
    bg_palette: u8,
    obj_palette: [u8; 2],
    win_y: u8,
    win_x: u8,

    bg_colors: [u32; 4],
    sp1_colors: [u32; 4],
    sp2_colors: [u32;  4],
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
            lcds: 0,
        }
    }
}

enum LcdMode {
    Hblack,
    Vblank,
    Oam,
    Xfer,
}

#[repr(u8)]
enum StatSrc {
    HBlank = 1 << 3,
    VBlank = 1 << 4,
    Oam = 1 << 5,
    Lyc = 1 << 6,
}

impl Bus {
    pub fn lcd_read(&self, address: u16) -> u8 {
        let offset = (address - 0xFF40) as usize;
        let p = &self.lcd as *const Lcd as *const u8;

        unsafe { *p.add(offset) }
    }

    pub fn update_palette(&mut self, palette_data: u8, pal: u8) {
        let p_colors = match pal {
            1 => &mut self.lcd.sp1_colors,
            2 => &mut self.lcd.sp2_colors,
            _ => &mut self.lcd.bg_colors,
        };

        p_colors[0] = DEFAULT_COLRORS[(palette_data & 0b11) as usize];
        p_colors[1] = DEFAULT_COLRORS[((palette_data >> 2) & 0b11) as usize];
        p_colors[2] = DEFAULT_COLRORS[((palette_data >> 4) & 0b11) as usize];
        p_colors[3] = DEFAULT_COLRORS[((palette_data >> 6) & 0b11) as usize];
    }

    pub fn lcd_write(&mut self, address: u16, value: u8) {
        let offset = (address - 0xFF40) as usize;
        let lcd_ref = &mut self.lcd;
        let p = lcd_ref as *mut Lcd as *mut u8;

        unsafe { *p.add(offset) = value; }

        if offset == 6 {
            // 0xFF46 = DMA
            self.dma_start(value);
        }

        match address {
            0xFF47 => self.update_palette(value, 0),
            0xFF48 => self.update_palette(value & 0b11111100, 1),
            0xFF49 => self.update_palette(value & 0b11111100, 2),
            _ => {}
        }
    }

}

impl Lcd {


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
