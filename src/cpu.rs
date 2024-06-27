mod register;
mod decode;
mod execute;

use crate::cpu::register::{Register8, Register16, RegisterData};

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

    pub fn prefetch_next(&mut self, addr: u16) -> Step {

    }

    pub fn execute_step(&mut self, step: Step) -> Step{
    }
}
