mod register;
mod decode;
mod execute;

use crate::cpu::register::{Register8, Register16, RegisterData};

pub struct Cpu {
    pub registers: register::RegisterData,
    pub opcode: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: RegisterData::new(),
            opcode: 0x00,
        }
    }
}
