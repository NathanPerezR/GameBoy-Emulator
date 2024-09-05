use crate::timer::Timer;
use crate::cpu::Cpu;
use crate::dma::Dma;
use crate::lcd::Lcd;

#[derive(Copy, Clone, Default, Debug)]
pub struct Io {
    serial: [u8; 2],
    ly: u8,
    pub timer: Timer,
}

impl Io {
    pub fn read(&mut self, address: u16, cpu: &Cpu,  lcd: &Lcd) -> u8 {
        match address {
            0xFF01 => self.serial[0],
            0xFF02 => self.serial[1],
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => cpu.get_interrupt_flags(),
            0xFF44 => {let tmp = self.ly; self.ly = self.ly.wrapping_add(1); tmp},
            0xFF40..=0xFF4B => lcd.read(address),
            _ => {
                println!("UNSUPPORTED read in io({:04X})", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8, cpu: &mut Cpu, lcd: &mut Lcd, dma: &mut Dma) {
        match address {
            0xFF01 => self.serial[0] = value,
            0xFF02 => self.serial[1] = value,
            0xFF04..=0xFF07 => self.timer.write(address, value),
            0xFF0F => cpu.set_interrupt_flags(value),
            0xFF46 => dma.start(value), 
            0xFF40..=0xFF4B => lcd.write(address, value, dma),
            _ => {
                println!("UNSUPPORTED write in io({:04X})", address);
            }
        }
    }
}


