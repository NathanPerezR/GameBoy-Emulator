mod decode;
mod execute;
mod register;
mod modes;
use crate::cpu::register::RegisterType;
use crate::bus::bus_read;
use crate::cart::Cart;
use crate::util::*;

#[derive(Clone,Copy,Debug,Default)]
enum AddressMode {
    #[default]
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

#[derive(Clone,Copy,Debug,Default)]
enum ConditionType {
    #[default]
    CtNone,
    CtNz,
    CtZ,
    CtNc,
    CtC,
}


#[derive(Clone, Copy, Debug, Default)]
pub struct Cpu {
    pub registers: register::RegisterData,
    pub cpu_ctx: CpuContext,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone,Copy,Debug)]
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
            mode: AddressMode::AmImp,
            register_1: RegisterType::default(),
            register_2: RegisterType::default(),
            condition: ConditionType::CtNone,
            parmater: 0,
        }
    }
}

impl Cpu {
    
    fn fetch_opcode(&mut self, cart: &Cart) {
        self.cpu_ctx.current_opcode = bus_read(cart, self.registers.pc);
        self.registers.pc += 1;
        self.cpu_ctx.current_instruction = self.instruction_by_opcode();
    }

    // fetching data depends on the current mode of the CPU
    fn fetch_data(&mut self, cart: &Cart) {
        self.cpu_ctx.mem_dest = 0;
        self.cpu_ctx.dest_is_mem = 0;
        
        use AddressMode::*;
        match &self.cpu_ctx.current_instruction.mode {
            AmImp => (),                                
            AmR => {                                                
                self.cpu_ctx.fetched_data = self.read_reg(self.cpu_ctx.current_instruction.register_1);
            },
            AmRD8 => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            AmD16 => {
                let lo: u8 = bus_read(cart, self.registers.pc);
                self.emu_cycles(1);

                let hi: u8 = bus_read(cart, self.registers.pc + 1);
                self.emu_cycles(1);
                self.registers.pc += 2;

                self.cpu_ctx.fetched_data = ((hi as u16) << 8) | (lo as u16); 
            },
            _ => { 
                println!("Unknown Addressing mode hit");
            }
        }
    }
    

    fn execute(&mut self, cart: &Cart) {
        self.decode_exe_fetch();
    }

    fn emu_cycles(&self, cycle: u8) {
        println!("need to do: emu emu_cycles")
    }

    pub fn step(&mut self, cart: &Cart) -> bool {
        if !self.cpu_ctx.halted {

            let pc: u16 = self.registers.pc;

            self.fetch_opcode(cart);
            self.fetch_data(cart);

            println!("PC: {:04X} Executing instruction: {:02X} ({:02X} {:02X} {:02X}) A:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} F:{:02X} H:{:02X} L:{:02X} | fetched-data: {:04X} ", 
                    pc,
                    self.cpu_ctx.current_opcode,
                    bus_read(cart, pc),
                    bus_read(cart, pc + 1),
                    bus_read(cart, pc + 2),
                    self.registers.a,
                    self.registers.b,
                    self.registers.c,
                    self.registers.d,
                    self.registers.e,
                    self.registers.f,
                    self.registers.h,
                    self.registers.l,
                    self.cpu_ctx.fetched_data,
                );

            self.execute(cart);
        }
        true
    }

    pub fn read_reg(&self, register_type: RegisterType) -> u16 {
        use RegisterType::*;
        match register_type {
            A => self.registers.a.into(),
            B => self.registers.b.into(),
            C => self.registers.c.into(),
            D => self.registers.d.into(),
            E => self.registers.e.into(),
            F => self.registers.f.into(),
            H => self.registers.h.into(),
            L => self.registers.l.into(),
            AF => (self.registers.a as u16) << 8 | (self.registers.f as u16),
            BC => (self.registers.b as u16) << 8 | (self.registers.c as u16),
            DE => (self.registers.d as u16) << 8 | (self.registers.e as u16),
            HL => (self.registers.h as u16) << 8 | (self.registers.l as u16), 
            PC => self.registers.pc, 
            SP => self.registers.sp,
        } 
    }

    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        if z {
            self.registers.f = bit_set(self.registers.f, 7, true);
        }
        if n {
            self.registers.f = bit_set(self.registers.f, 6, true);
        }
        if h {
            self.registers.f = bit_set(self.registers.f, 5, true);
        }
        if c {
            self.registers.f = bit_set(self.registers.f, 4, true);
        }
    }

    pub fn check_cond(&mut self) -> bool {

        let c_flag: bool = nth_bit(self.registers.f.into(), 7);
        let z_flag: bool = nth_bit(self.registers.f.into(), 4);

        use ConditionType::*;
        match self.cpu_ctx.current_instruction.condition {
            CtNone => true,
            CtZ => z_flag,
            CtNz => !z_flag,
            CtC => c_flag,
            CtNc => !c_flag,
        }
    }


}
