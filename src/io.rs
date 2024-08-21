use crate::timer::Timer;
use crate::cpu::Cpu;

#[derive(Copy, Clone, Default, Debug)]
pub struct Io {
    serial: [u8; 2],
    pub timer: Timer,
}

impl Io {
    pub fn read(self, address: u16, cpu: &Cpu) -> u8 {
        match address {
            0xFF01 => self.serial[0],
            0xFF02 => self.serial[1],
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF0F => cpu.get_interrupt_flags(),
            _ => {
                println!("UNSUPPORTED read in io({:04X})", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8, cpu: &mut Cpu) {
        match address {
            0xFF01 => self.serial[0] = value,
            0xFF02 => self.serial[1] = value,
            0xFF04..=0xFF07 => self.timer.write(address, value),
            0xFF0F => cpu.set_interrupt_flags(value),
            _ => {
                println!("UNSUPPORTED write in io({:04X})", address);
            }
        }
    }
}


