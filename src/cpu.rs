mod execute;
mod register;
mod modes;
mod structs;
mod cpu_fetch;
use structs::InstructionName;

use crate::interrupts::InterruptType;
use crate::cpu::register::RegisterType;
use crate::bus::Bus;
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
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
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
        self.cpu_ctx.current_opcode = bus.read(self.pc, self);
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

    pub fn step(&mut self, bus: &mut Bus, dbg: &mut Debugger) -> bool {
        if !self.cpu_ctx.halted {

            // if self.a == 0x21 && self.f == 0x10 && self.c == 0x0E {
            //     println!("hit");
            //     println!("bang")
            // }

            let inst = self.inst_to_str(bus);
            println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X}) {}",
            self.a,
            self.f,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,

            bus.read(self.pc.wrapping_add(0), self),
            bus.read(self.pc.wrapping_add(1), self),
            bus.read(self.pc.wrapping_add(2), self),
            bus.read(self.pc.wrapping_add(3), self),
            inst
        );

            self.fetch_opcode(bus);
            self.fetch_data(bus);

                let flags = format!(
                    "{}{}{}{}",
                    if self.f & (1 << 7) != 0 { 'Z' } else { '-' },
                    if self.f & (1 << 6) != 0 { 'N' } else { '-' },
                    if self.f & (1 << 5) != 0 { 'H' } else { '-' },
                    if self.f & (1 << 4) != 0 { 'C' } else { '-' }
                );
        
                // let inst = self.inst_to_str(bus);
        
                // println!(
                //     "{:04X}: {:<12} ({:02X} {:02X} {:02X}) A:{:02X} F:{} BC:{:02X}{:02X} DE:{:02X}{:02X} HL:{:02X}{:02X} | Fetched Data: {:04X}",
                //     pc,
                //     inst,
                //     self.cpu_ctx.current_opcode,
                //     bus.read(pc + 1, self),
                //     bus.read(pc + 2, self),
                //     self.a,
                //     flags,
                //     self.b,
                //     self.c,
                //     self.d,
                //     self.e,
                //     self.h,
                //     self.l,
                //     self.cpu_ctx.fetched_data,
                // );


            // DEBUG INFO FOR BLARG 
            dbg.update(self, bus);
            self.dbg.print();
        

            self.execute(bus);

        }
        else {
            self.emu_cycles(1, bus);

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

        let c_flag: bool = self.get_c();
        let z_flag: bool = self.get_z();

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
        let popped_value = bus.read(self.sp, self);
        self.sp = self.sp.wrapping_add(1);
        popped_value
    }

pub fn stack_pop16(&mut self, bus: &mut Bus) -> u16 {
    let lo: u16 = self.stack_pop(bus).into();
    let hi: u16 = self.stack_pop(bus).into();

    (lo) | (hi << 8)
}


    pub fn emu_cycles(&mut self, cpu_cycles: u16, bus: &mut Bus) {
        let n: u32 = (cpu_cycles * 4).into();

        for _ in 0..n {
            {
                bus.io.timer.tick(self);
            }
        }
    }

    pub fn set_ie_register(&mut self, n: u8) {
        self.cpu_ctx.ie_register = n;
    }

    pub fn get_ie_register(&self) -> u8 {
        self.cpu_ctx.ie_register 
    }

    pub fn get_interrupt_flags(&self) -> u8 {
        self.cpu_ctx.interrupt_flag
    }

    pub fn set_interrupt_flags(&mut self, value: u8) {
        self.cpu_ctx.interrupt_flag = value;
    }

    pub fn request_interrupt(&mut self, t: InterruptType) {
        self.cpu_ctx.interrupt_flag |= t as u8;
    }

    pub fn inst_to_str(&self, bus: &mut Bus) -> String {
        let inst = &self.cpu_ctx.instruction;
        let inst_name = self.inst_name(inst.in_type);
        let mut result = format!("{} ", inst_name);

        match inst.mode {
            AddressMode::Imp => return result.trim().to_string(),

            AddressMode::RD16 | AddressMode::RA16 => {
                format!(
                    "{} {}${:04X}",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.cpu_ctx.fetched_data
                )
            }

            AddressMode::R => {
                format!(
                    "{} {}",
                    inst_name,
                    self.rt_lookup(inst.register_1)
                )
            }

            AddressMode::RR => {
                format!(
                    "{} {},{}",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                )
            }

            AddressMode::MrR => {
                format!(
                    "{} ({}, {})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                )
            }

            AddressMode::Mr => {
                format!(
                    "{} ({})",
                    inst_name,
                    self.rt_lookup(inst.register_1)
                )
            }

            AddressMode::RMr => {
                format!(
                    "{} {}, ({})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                )
            }

            AddressMode::RD8 | AddressMode::RA8 => {
                format!(
                    "{} {}${:02X}",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.cpu_ctx.fetched_data & 0xFF
                )
            }

            AddressMode::RHli => {
                result = format!(
                    "{} {},({}+)",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            AddressMode::RHld => {
                result = format!(
                    "{} {},({}-)",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            AddressMode::HliR => {
                result = format!(
                    "{} ({:+}, {})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            AddressMode::HldR => {
                result = format!(
                    "{} ({:+}, {})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            AddressMode::A8R => {
                result = format!(
                    "{} ${:02X}, {}",
                    inst_name,
                    bus.read(self.pc - 1, self),
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            AddressMode::HlSpr => {
                result = format!(
                    "{} ({}, SP+{:})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.cpu_ctx.fetched_data & 0xFF
                );
                return result;
            }

            AddressMode::D8 => {
                result = format!(
                    "{} ${:02X}",
                    inst_name,
                    self.cpu_ctx.fetched_data & 0xFF
                );
                return result;
            }

            AddressMode::D16 => {
                result = format!(
                    "{} ${:04X}",
                    inst_name,
                    self.cpu_ctx.fetched_data
                );
                return result;
            }

            AddressMode::MrD8 => {
                result = format!(
                    "{} ({}, ${:02X})",
                    inst_name,
                    self.rt_lookup(inst.register_1),
                    self.cpu_ctx.fetched_data & 0xFF
                );
                return result;
            }

            AddressMode::A16R => {
                result = format!(
                    "{} (${:04X}, {})",
                    inst_name,
                    self.cpu_ctx.fetched_data,
                    self.rt_lookup(inst.register_2)
                );
                return result;
            }

            _ => {
                eprintln!("INVALID AM: {:?}", inst.mode);
                return "INVALID".to_string();
            }
        }
    }

    fn inst_name(&self, inst_type: InstructionName) -> &'static str {
        match inst_type {
            InstructionName::None => "None",
            InstructionName::Nop => "NOP",
            InstructionName::Ld => "LD",
            InstructionName::Inc => "INC",
            InstructionName::Dec => "DEC",
            InstructionName::Rlca => "RLCA",
            InstructionName::Add => "ADD",
            InstructionName::Rrca => "RRCA",
            InstructionName::Stop => "STOP",
            InstructionName::Rla => "RLA",
            InstructionName::Jr => "JR",
            InstructionName::Rra => "RRA",
            InstructionName::Daa => "DAA",
            InstructionName::Cpl => "CPL",
            InstructionName::Scf => "SCF",
            InstructionName::Ccf => "CCF",
            InstructionName::Halt => "HALT",
            InstructionName::Adc => "ADC",
            InstructionName::Sub => "SUB",
            InstructionName::Sbc => "SBC",
            InstructionName::And => "AND",
            InstructionName::Xor => "XOR",
            InstructionName::Or => "OR",
            InstructionName::Cp => "CP",
            InstructionName::Pop => "POP",
            InstructionName::Jp => "JP",
            InstructionName::Push => "PUSH",
            InstructionName::Ret => "RET",
            InstructionName::Call => "CALL",
            InstructionName::Reti => "RETI",
            InstructionName::Ldh => "LDH",
            InstructionName::Jphl => "JPHL",
            InstructionName::Di => "DI",
            InstructionName::Ei => "EI",
            InstructionName::Rst => "RST",
            InstructionName::Err => "ERR",
            InstructionName::Rlc => "RLC",
            InstructionName::Rrc => "RRC",
            InstructionName::Rl => "RL",
            InstructionName::Rr => "RR",
            InstructionName::Sla => "SLA",
            InstructionName::Sra => "SRA",
            InstructionName::Swap => "SWAP",
            InstructionName::Srl => "SRL",
            InstructionName::Bit => "BIT",
            InstructionName::Res => "RES",
            InstructionName::Set => "SET",
            InstructionName::CB => "CB",
        }
    }
}
