mod decode;
mod execute;
mod register;
use crate::cpu::register::{Register8, Register16};
use crate::bus::bus_read;
use crate::cart::Cart;

#[derive(Debug)]
enum AddressMode{
    AmImp,
    AmRD16,
    AmRR,
    AmMrR,
    AmR,
    AmRD8,
    AmRMr,
    AmRHli,
    AmRHld,
    AmHliR,
    AmHldR,
    AmRA8,
    AmA8R,
    AmHlSpr,
    AmD16,
    AmD8,
    AmD16R,
    AmMrD8,
    AmMr,
    AmA16R,
    AmRA16,
}

enum ConditionType {
    CtNone,
    CtNz,
    CtZ,
    CtNc,
    CtC,
}

enum RegisterType {
    Register8(Register8), 
    Register16(Register16),
}

pub struct Cpu {
    pub registers: register::RegisterData,
    pub cpu_ctx: CpuContext,
}

struct CpuContext {
    fetched_date: u16,
    mem_dest: u16,
    dest_is_mem: u16,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool, 
    stepping: bool,
    int_master_enabled: bool,
}


struct Instruction {
    in_type: u8,
    mode: AddressMode, 
    register_1: AddressMode, 
    register_2: AddressMode,
    condition: ConditionType, 
    parmater: u8,
} 

impl Cpu {
    
    fn fetch_instruction(&mut self, cart: Cart) {
        self.cpu_ctx.current_opcode = bus_read(cart , self.registers.pc); 
        self.cpu_ctx.current_instruction = instruction_by_opcode(self.cpu_ctx.current_opcode);
    }

    fn fetch_data(& self) {

    }

    fn execute() {

    }

    fn cpu_step(&mut self, cart: Cart) -> bool {
        
        if !self.cpu_ctx.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instruction();
            self.fetch_data();

            println!("{:04X}: {:<7} ({:02X} {:02X} {:02X}) A: {:02X} B: {:02X} C: {:02X}", 
                     self.registers.pc,
                     5,
                     self.current_opcode,
                     bus_read(cart, self.registers.pc + 1),
                     bus_read(cart, self.registers.pc + 2),
                     self.registers.a,
                     self.registers.b,
                     self.registers.c,
                    );
            
            self.execute();
        }

        true
    }
}
