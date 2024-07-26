mod decode;
mod execute;
mod register;
use crate::cpu::register::RegisterType;
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


#[derive(Default)]
pub struct Cpu {
    pub registers: register::RegisterData,
    pub cpu_ctx: CpuContext,
}

#[derive(Default)]
struct CpuContext {
    fetched_data: u16,
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
    register_1: RegisterType, 
    register_2: RegisterType,
    condition: ConditionType, 
    parmater: u8,
} 

//TODO: find the settings in CPU
impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            in_type: 0,
            mode: AddressMode::AmR,
            register_1: RegisterType::default(),
            register_2: RegisterType::default(),
            condition: ConditionType::CtNone,
            parmater: 0,
        }
    }
}

impl Cpu {
    
    fn fetch_instruction(&mut self, cart: &Cart) {
        self.cpu_ctx.current_opcode = bus_read(cart , self.registers.pc);
        self.registers.pc = self.registers.pc + 1;
        self.cpu_ctx.current_instruction = instruction_by_opcode(self.cpu_ctx.current_opcode);
    }

    // fetching data depends on the current mode of the CPU
    fn fetch_data(&mut self, cart: &Cart) {
        self.cpu_ctx.mem_dest = 0;
        self.cpu_ctx.dest_is_mem = 0;

        match &self.cpu_ctx.current_instruction.mode {
            AmImp => return,                                
            AmR => {                                                
                self.cpu_ctx.fetched_data = self.cpu_read_reg(self.cpu_ctx.current_instruction.register_1);
                return
            },
            AmRD8 => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc);
                self.emu_cycles(1);
                self.registers.pc += 1;
                return
            },
            AmD16 => {
                let lo: u8 = bus_read(cart, self.registers.pc);
                self.emu_cycles(1);

                let hi: u8 = bus_read(cart, self.registers.pc + 1);
                self.registers.pc += 2;

                return
            },
            _ => { 
                println!("Unknown Addressing mode hit");
                return
            }
        }
    }
    

    fn execute(&self) {
        println!("need to do: executing")
    }

    fn emu_cycles(&self, cycle: u8) {
        println!("need to do: emu emu_cycles")
    }

    fn cpu_read_reg() {

    }

    pub fn step(&mut self, cart: &Cart) -> bool {
        
        if !self.cpu_ctx.halted {
            let pc: u16 = self.registers.pc;

            self.fetch_instruction(&cart);
            self.fetch_data(cart);

            println!("{:04X}: {:<7} ({:02X} {:02X} {:02X}) A: {:02X} B: {:02X} C: {:02X}", 
                     self.registers.pc,
                     5,
                     self.cpu_ctx.current_opcode,
                     bus_read(&cart, self.registers.pc + 1),
                     bus_read(&cart, self.registers.pc + 2),
                     self.registers.a,
                     self.registers.b,
                     self.registers.c,
                    );
            
            self.execute();
        }

        true
    }

    pub fn cpu_read_reg(&self, register_type: RegisterType) {
        return match register_type {
            A => self.registers.a,
            B => self.registers.b,
            C => self.registers.c,
            D => self.registers.d,
            E => self.registers.e,
            F => self.registers.f,
            AF => (self.registers.a as u16) << 8 | (self.registers.f as u16),
            BC => (self.registers.b as u16) << 8 | (self.registers.c as u16),
            DE => (self.registers.d as u16) << 8 | (self.registers.e as u16),
            HL => (self.registers.h as u16) << 8 | (self.registers.l as u16), 
            PC => self.registers.pc, 
            SP => self.registers.sp,
        } 
    }

}
