use crate::timer::Timer;
use crate::cpu::Cpu;
use crate::dma::Dma;

#[derive(Copy, Clone, Default, Debug)]
pub struct Io {
    serial: [u8; 2],
    ly: u8,
    pub timer: Timer,
}

impl Io {
    pub fn read(&mut self, address: u16, cpu: &Cpu) -> u8 {
        match address {
            0xFF01 => self.serial[0],
            0xFF02 => self.serial[1],
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => cpu.get_interrupt_flags(),
            0xFF44 => {let tmp = self.ly; self.ly += 1; return tmp;},
            _ => {
                // println!("UNSUPPORTED read in io({:04X})", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8, cpu: &mut Cpu, dma: &mut Dma) {
        match address {
            0xFF01 => self.serial[0] = value,
            0xFF02 => self.serial[1] = value,
            0xFF04..=0xFF07 => self.timer.write(address, value),
            0xFF0F => cpu.set_interrupt_flags(value),
            0xFF46 => dma.start(value), 
            _ => {
                // println!("UNSUPPORTED write in io({:04X})", address);
            }
        }
    }
}


