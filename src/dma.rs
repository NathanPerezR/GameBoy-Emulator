
use std::thread::sleep;
use std::time::Duration;
use crate::bus::Bus;
use crate::cpu::Cpu;

#[derive(Clone)]
pub struct Dma {
    active: bool,
    byte: u8,
    value: u8,
    start_delay: u8,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            active: true,
            byte: 0,
            value: 0,
            start_delay: 2,
        }
    }


    pub fn transferring(&self) -> bool {
        self.active
    }

    pub fn tick(&mut self, bus: &mut Bus, cpu: &Cpu) {
        if !self.active {
            return;
        }

        if self.start_delay > 0 {
            self.start_delay -= 1;
            return;
        }

        let tmp = bus.read((self.value as u16 * 0x100) + self.byte as u16, cpu);
        bus.ppu.oam_write(self.byte.into(), tmp);

        self.byte += 1;

        self.active = self.byte < 0xA0;

        if !self.active {
            // remove later
            sleep(Duration::from_millis(2));
        }
    }

    pub fn start(&mut self, start: u8) {
        self.active = true;
        self.byte = 0;
        self.start_delay = 2;
        self.value = start;
    }
}
