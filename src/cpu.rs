mod register;
mod decode;
mod execute;

use crate::cpu::register::{Register5, Register16, RegisterData};

pub struct Cpu {
    pub registers: register::RegisterData,
    pub opcode: u8,
}

pub enum Step {
    Running,
    Halt,
    InterruptDispatch,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: RegisterData::new(),
            opcode: 0x00,
        }
    }

    pub fn step(&mut self) -> bool {
    }

    pub fn init(&mut self) {

    }
}
