mod execute;
mod register;
mod modes;
mod structs;
mod cpu_fetch;
use crate::interrupts::InterruptType;
use crate::cpu::register::RegisterType;
use crate::bus::Bus;
use crate::util::nth_bit;
use crate::cpu::structs::{AddressMode, ConditionType, Instruction};
use crate::dbg::Debugger;

#[derive(Clone, Copy, Debug)]
pub struct Cpu {
    pub cpu_ctx: CpuContext,
    pub dbg: Debugger,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            cpu_ctx: CpuContext::default(),
            dbg: Debugger::new(),
            a: 0xB0,
            f: 0x01,
            b: 0x13,
            c: 0x00,
            d: 0xD8,
            e: 0x00,
            h: 0x4D,
            l: 0x01,
            pc: 0x100,
            sp: 0xFFFE,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CpuContext {
    pub fetched_data: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
    pub current_opcode: u8,
    pub instruction: Instruction,
    pub halted: bool, 
    pub stepping: bool,
    pub int_master_enabled: bool,
    pub enable_ime: bool,
    pub interrupt_flag: u8,
    pub ie_register: u8,
}

impl Cpu {
    
    fn fetch_opcode(&mut self, bus: &mut Bus) {
        self.cpu_ctx.current_opcode = bus.read(self.pc, *self);
        self.pc += 1;
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
    fn emu_cycles(&self, _cycle: u8) {
    }

    pub fn step(&mut self, bus: &mut Bus) -> bool {
        if !self.cpu_ctx.halted {

            let pc: u16 = self.pc;

            self.fetch_opcode(bus);
            self.fetch_data(bus);
            println!("PC: {:04X} | Executing instruction: {:5} ({:02X} {:02X} {:02X}) A:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} F:{:02X} H:{:02X} L:{:02X} SP:{:04X} | fetched-data: {:04X} ", 
                    pc,
                    format!("{:?}", self.cpu_ctx.instruction.in_type).chars().take(5).collect::<String>(),
                    bus.read(pc, *self),
                    bus.read(pc + 1, *self),
                    bus.read(pc + 2, *self),
                    self.a,
                    self.b,
                    self.c,
                    self.d,
                    self.e,
                    self.f,
                    self.h,
                    self.l,
                    self.sp,
                    self.cpu_ctx.fetched_data,
                );

            // DEBUG INFO FOR BLARG 
            self.dbg.update(*self, bus);
            self.dbg.print();


            self.execute(bus);

        }
        else {
            self.emu_cycles(1);

            if self.cpu_ctx.interrupt_flag != 0 {
                self.cpu_ctx.halted = false;
            }

        }

        if self.cpu_ctx.int_master_enabled {
            self.handle_interrupt(bus);
            self.cpu_ctx.enable_ime = false;
        }

        if self.cpu_ctx.enable_ime {
            self.cpu_ctx.int_master_enabled = true;
        }
        true
    }


    pub fn check_cond(&mut self) -> bool {

        let c_flag: bool = nth_bit(self.f.into(), 7);
        let z_flag: bool = nth_bit(self.f.into(), 4);

        use ConditionType::*;
        match self.cpu_ctx.instruction.condition {
            None => true,
            Z => z_flag,
            Nz => !z_flag,
            C => c_flag,
            Nc => !c_flag,
        }
    }

    pub fn stack_push(&mut self, bus: &mut Bus, data: u8) {
        
        self.sp = self.sp.wrapping_sub(1);
        bus.write(self.sp, data, self); 

    }

    pub fn stack_push16(&mut self, bus: &mut Bus, data: u16) {
        self.stack_push(bus, (data >> 8) as u8);
        self.stack_push(bus, data as u8);
    }

    pub fn stack_pop(&mut self, bus: &mut Bus) -> u8 {
        let popped_value = bus.read(self.sp, *self);
        self.sp = self.sp.wrapping_add(1);
        popped_value
    }

    pub fn stack_pop16(&mut self, bus: &mut Bus) -> u16 {
        let lo: u16 = self.stack_pop(bus).into();
        let hi: u16 = self.stack_pop(bus).into();

        (hi << 8) | lo
    }

    pub fn set_ie_register(&mut self, n: u8) {
        self.cpu_ctx.ie_register = n;
    }

    pub fn get_ie_register(self) -> u8 {
        self.cpu_ctx.ie_register 
    }

    pub fn get_interrupt_flags(self) -> u8 {
        self.cpu_ctx.interrupt_flag
    }

    pub fn set_interrupt_flags(&mut self, value: u8) {
        self.cpu_ctx.interrupt_flag = value;
    }

    pub fn request_interrupt(& mut self, t: InterruptType) {
        self.cpu_ctx.interrupt_flag |= t as u8;
    }
}
