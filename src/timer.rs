use crate::cpu::Cpu;
use crate::interrupts::InterruptType;


#[derive(Copy, Clone, Debug)]
pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            div: 0xAC00,
            tima: 0,
            tma: 0,
            tac: 0
        }
    }
}

impl Timer {

    pub fn tick(&mut self, cpu: &mut Cpu) {
        let prev_div = self.div;
        self.div = self.div.wrapping_add(1);

        let timer_update = match self.tac & 0b11 {
            0b00 => (prev_div & (1 << 9)) != 0 && (self.div & (1 << 9)) == 0,
            0b01 => (prev_div & (1 << 3)) != 0 && (self.div & (1 << 3)) == 0,
            0b10 => (prev_div & (1 << 5)) != 0 && (self.div & (1 << 5)) == 0,
            0b11 => (prev_div & (1 << 7)) != 0 && (self.div & (1 << 7)) == 0,
            _ => false,
        };

        if timer_update && (self.tac & (1 << 2)) != 0 {
            self.tima = self.tima.wrapping_add(1);

            if self.tima == 0xFF {
                self.tima = self.tma;
                cpu.request_interrupt(InterruptType::Timer);
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.tac = value,
            _ => {},
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => 0,
        }
    }
}

