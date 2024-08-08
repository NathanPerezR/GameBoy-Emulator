mod execute;
mod register;
mod modes;
mod structs;
mod cpu_fetch;
use crate::cpu::register::RegisterType;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::util::nth_bit;
use crate::cpu::structs::{AddressMode, ConditionType, Instruction};

#[derive(Clone, Copy, Debug, Default)]
pub struct Cpu {
    pub registers: register::RegisterData,
    pub cpu_ctx: CpuContext,
}

#[derive(Clone, Copy, Debug, Default)]
struct CpuContext {
    fetched_data: u16,
    mem_dest: u16,
    dest_is_mem: bool,
    current_opcode: u8,
    instruction: Instruction,
    halted: bool, 
    stepping: bool,
    int_master_enabled: bool,
}

impl Cpu {
    
    fn fetch_opcode(&mut self, bus: &mut Bus) {
        self.cpu_ctx.current_opcode = bus.read(self.registers.pc);
        self.registers.pc += 1;
        self.instruction_by_opcode();
    }

    fn execute(&mut self, bus: &mut Bus) {

        if let Some(func) = self.cpu_ctx.instruction.function {
            func(self, bus);
        } else {
            self.cpu_ctx.halted = true;
            println!("NO VALID INSTRUCTION FOUND")
        }
    }

    // TODO: NEED TO DO EMU CYCLES
    fn emu_cycles(&self, cycle: u8) {
    }

    pub fn step(&mut self, bus: &mut Bus) -> bool {
        if !self.cpu_ctx.halted {

            let pc: u16 = self.registers.pc;

            self.fetch_opcode(bus);
            self.fetch_data(bus);
            println!("PC: {:04X} | Executing instruction: {:5} ({:02X} {:02X} {:02X}) A:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} F:{:02X} H:{:02X} L:{:02X} SP:{:04X} | fetched-data: {:04X} ", 
                    pc,
                    format!("{:?}", self.cpu_ctx.instruction.in_type).chars().take(5).collect::<String>(),
                    bus.read(pc),
                    bus.read(pc + 1),
                    bus.read(pc + 2),
                    self.registers.a,
                    self.registers.b,
                    self.registers.c,
                    self.registers.d,
                    self.registers.e,
                    self.registers.f,
                    self.registers.h,
                    self.registers.l,
                    self.registers.sp,
                    self.cpu_ctx.fetched_data,
                );
            self.execute(bus);

        }
        true
    }


    pub fn check_cond(&mut self) -> bool {

        let c_flag: bool = nth_bit(self.registers.f.into(), 7);
        let z_flag: bool = nth_bit(self.registers.f.into(), 4);

        use ConditionType::*;
        match self.cpu_ctx.instruction.condition {
            None => true,
            Z => z_flag,
            Nz => !z_flag,
            C => c_flag,
            Nc => !c_flag,
        }
    }
}
