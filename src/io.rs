use crate::timer::Timer;
use crate::cpu::Cpu;
use crate::bus::Bus;

#[derive(Copy, Clone, Default, Debug)]
pub struct Io {
    serial: [u8; 2],
    ly: u8,
    pub timer: Timer,
}

impl Bus {
    pub fn io_read(&mut self, address: u16, cpu: &Cpu) -> u8 {
        match address {
            0xFF01 => self.io.serial[0],
            0xFF02 => self.io.serial[1],
            0xFF04..=0xFF07 => self.io.timer.read(address),
            0xFF0F => cpu.get_interrupt_flags(),
            0xFF44 => {let tmp = self.io.ly; self.io.ly = self.io.ly.wrapping_add(1); tmp},
            0xFF40..=0xFF4B => self.lcd_read(address),
            _ => {
                println!("UNSUPPORTED read in io({:04X})", address);
                0
            }
        }
    }

    pub fn io_write(&mut self, address: u16, value: u8, cpu: &mut Cpu) {
        match address {
            0xFF01 => self.io.serial[0] = value,
            0xFF02 => self.io.serial[1] = value,
            0xFF04..=0xFF07 => self.io.timer.write(address, value),
            0xFF0F => cpu.set_interrupt_flags(value),
            0xFF46 => self.dma_start(value), 
            0xFF40..=0xFF4B => self.lcd_write(address, value),
            _ => {
                println!("UNSUPPORTED write in io({:04X})", address);
            }
        }
    }
}


