use crate::bus::Bus;
use crate::lcd::{LcdMode, Lcd, StatSrc};
use crate::cpu::Cpu;
use crate::interrupts::InterruptType;
use crate::ui::UI;
use std::time::Duration;

#[derive(Clone)]
pub struct Ppu {
    
    
    // pub video_buffer: Vec<u32>,
    pub oam_ram: [OamEntry; 40],
    pub vram: [u8; 0x2000],


    current_frame: u32,
    line_ticks: u32,
    prev_frame_time: u32,
    start_timer: u32,
    frame_count: u32,
}


const LINES_PER_FRAME: u16 = 154;
const TICKS_PER_LINE: u16 = 456;
const YRES: u8 = 144;
const XRES: u8 = 160;

impl Ppu {

    pub fn new() -> Self {
        // let video_buffer = vec![0; (YRES * XRES) as usize]; 
        Ppu {
            current_frame: 0,
            line_ticks: 0,
            frame_count: 0,
            prev_frame_time: 0,
            start_timer: 0,
            // video_buffer,
            oam_ram: [OamEntry::new(0, 0, 0, 0); 40],
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

    pub fn tick(&mut self, lcd: &mut Lcd, cpu: &mut Cpu, ui: &UI) {
        self.line_ticks += 1;

        match LcdMode::try_from(lcd.get_mode()) {
            Ok(LcdMode::Oam) => self.mode_oam(lcd),
            Ok(LcdMode::Xfer) => self.mode_xfer(lcd),
            Ok(LcdMode::Vblank) => self.mode_vblank(lcd, cpu),
            Ok(LcdMode::Hblank) => self.mode_hblank(lcd, cpu, ui),
            Err(..) => unreachable!()
        }
    }

    fn mode_oam(&mut self, lcd: &mut Lcd) {
        if self.line_ticks >= 80 {
            lcd.lcds_mode_set(LcdMode::Xfer as u8);
        }
    }

    fn mode_xfer(&mut self, lcd: &mut Lcd) {
        if self.line_ticks >= (80 + 172) {
            lcd.lcds_mode_set(LcdMode::Hblank as u8);
        }
    }

    fn mode_vblank(&mut self, lcd: &mut Lcd, cpu: &mut Cpu, ) {
        if self.line_ticks >= TICKS_PER_LINE.into() {
            lcd.increment_ly(cpu);

            if u16::from(lcd.ly) >= LINES_PER_FRAME {
                lcd.lcds_mode_set(LcdMode::Oam as u8);
                lcd.ly = 0;
            }

            self.line_ticks = 0;
        }
    }

    const TARGET_FRAME_TIME: u32 = 1000 / 60;

    fn mode_hblank(&mut self, lcd: &mut Lcd, cpu: &mut Cpu, ui: &UI) {
        if self.line_ticks >= TICKS_PER_LINE.into() { 
            lcd.increment_ly(cpu);

            if lcd.ly >= YRES {
                lcd.lcds_mode_set(LcdMode::Vblank as u8);

                cpu.request_interrupt(InterruptType::VbBlank);

                if lcd.lcds_stat_int(StatSrc::VBlank as u8) {
                    cpu.request_interrupt(InterruptType::LcdStat);
                }

                self.current_frame += 1;


                let end = ui.get_ticks();
                let frame_time = end - self.prev_frame_time;

                if frame_time < Self::TARGET_FRAME_TIME {
                    self.delay(Self::TARGET_FRAME_TIME - frame_time);
                }

                if end - self.start_timer >= 1000 {
                    let fps = self.frame_count;
                    self.start_timer = end;
                    self.frame_count = 0;

                    println!("FPS: {}", fps);
                }

                self.frame_count += 1;
                self.prev_frame_time = ui.get_ticks();

            } else {
                lcd.lcds_mode_set(LcdMode::Oam as u8);
            }

            self.line_ticks = 0;
        }
    }

    pub fn delay(&self, ms: u32) {
        std::thread::sleep(Duration::from_millis(ms as u64));
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
