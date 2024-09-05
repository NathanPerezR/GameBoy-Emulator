
use std::thread::sleep;
use std::time::Duration;
use crate::bus::Bus;
use crate::cpu::Cpu;

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
}

impl Bus {
    pub fn dma_tick(&mut self, cpu: &Cpu) {
        if !self.dma.active {
            return;
        }

        if self.dma.start_delay > 0 {
            self.dma.start_delay -= 1;
            return;
        }

        let tmp = self.read((self.dma.value as u16 * 0x100) + self.dma.byte as u16, cpu);
        self.ppu.oam_write(self.dma.byte.into(), tmp);

        self.dma.byte += 1;

        self.dma.active = self.dma.byte < 0xA0;

        if !self.dma.active {
            // remove later
            sleep(Duration::from_millis(2));
        }
    }

    pub fn dma_start(&mut self, start: u8) {
        self.dma.active = true;
        self.dma.byte = 0;
        self.dma.start_delay = 2;
        self.dma.value = start;
    }
}
