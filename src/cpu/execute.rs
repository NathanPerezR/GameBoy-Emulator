use crate::bus::Bus;
use crate::cpu::{RegisterType, ConditionType};
use crate::cpu::register::is_16_bit;
use crate::cpu::structs::AddressMode;
use crate::cpu::Cpu;
use crate::emu;

impl Cpu {

    pub fn load(&mut self, bus: &mut Bus) {
        if self.cpu_ctx.dest_is_mem {

            use RegisterType::*;
            match self.cpu_ctx.instruction.register_2 {
                A | B | C | D | E | L | F | H => {
                    bus.write(self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data as u8, self);   
                },
                AF | BC | DE | HL | PC | SP => {
                    self.emu_cycles(1);
                    bus.write16(self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data, self);
                },
                _ => println!("Error, none selected")
            }
            return;
        }

        if let AddressMode::HlSpr = self.cpu_ctx.instruction.mode {
            let h_flag: bool = (self.read(self.cpu_ctx.instruction.register_2) & 0xF).wrapping_add 
                (self.cpu_ctx.fetched_data & 0xF) >= 0x10;
            let c_flag: bool = (self.read(self.cpu_ctx.instruction.register_2) & 0xFF).wrapping_add
                (self.cpu_ctx.fetched_data & 0xFF) >= 0x100;
            
            self.set_z(false);
            self.set_n(false);
            self.set_h(h_flag);
            self.set_c(c_flag);

            let value_reg_2 = self.read(self.cpu_ctx.instruction.register_2);
            let new_value = value_reg_2.wrapping_add(self.cpu_ctx.fetched_data);
            self.set_reg(self.cpu_ctx.instruction.register_1, new_value);
            return;
        }

        self.set_reg(self.cpu_ctx.instruction.register_1, self.cpu_ctx.fetched_data)
    }

    const RT_LOOKUP: [RegisterType; 8] = [
        RegisterType::B,
        RegisterType::C,
        RegisterType::D,
        RegisterType::E,
        RegisterType::H,
        RegisterType::L,
        RegisterType::HL,
        RegisterType::A
    ];

    fn decode_reg(reg: u8) -> RegisterType {
        if reg > 0b111 {
            RegisterType::None
        } else {
            Self::RT_LOOKUP[reg as usize]
        }
    }

    pub fn cb(&mut self, bus: &mut Bus) {
        let op: u8 = self.cpu_ctx.fetched_data.try_into().unwrap();
        let reg: RegisterType = Self::decode_reg(op & 0b111);
        let bit: u8 = (op >> 3) & 0b111;
        let bit_op: u8 = (op >> 6) & 0b11;
        let mut reg_val: u8 = self.read_8(bus, reg);

        self.emu_cycles(1);

        if let RegisterType::HL = reg {
            self.emu_cycles(2);
        }

        match bit_op {
            1 => {
                self.set_z((reg_val & (1 << bit)) == 0);
                self.set_n(false);
                self.set_h(true);
                return;
            },
            2 => {
                reg_val &= !(1 << bit);
                self.set_reg_8(bus, reg, reg_val);
                return;
            }
            3 => {
                reg_val |= 1 << bit;
                self.set_reg_8(bus, reg, reg_val);
                return;
            }
            _ => {}
        }



        let c_flag: bool = self.get_c();

        match bit {
            0 => {
                let set_c = (reg_val & (1 << 7)) != 0;
                let result = (reg_val << 1) & 0xFF;
                let result = if set_c { result | 1 } else { result };
                self.set_reg_8(bus, reg, result);

                self.set_z(result == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(set_c);
            },
            1 => {

                let old = reg_val;
                reg_val = (reg_val >> 1) | (old << 7);
                self.set_reg_8(bus, reg, reg_val);
                self.set_z(reg_val == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(old & 1 != 0);
            },
            2 => {
                let old = reg_val;
                reg_val = (reg_val << 1) | (if c_flag { 1 } else { 0 });
                self.set_reg_8(bus, reg, reg_val);
                self.set_z(reg_val == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c((old & 0x80) != 0);
            },
            3 => {
                let old = reg_val;
                reg_val = (reg_val >> 1) | (if c_flag { 0x80 } else { 0 });
                self.set_reg_8(bus, reg, reg_val);
                self.set_z(reg_val == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(old & 1 != 0);
            },
            4 => {
                let old = reg_val;
                reg_val <<= 1;
                self.set_reg_8(bus, reg, reg_val);
                self.set_z(reg_val == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c((old & 0x80) != 0);
            },
            5 => {

                let u = ((reg_val as i8) >> 1) as u8;
                self.set_reg_8(bus, reg, u);
                self.set_z(u == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(reg_val & 1 != 0);

            },
            6 => {
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0xF) << 4);
                self.set_reg_8(bus, reg, reg_val);
                self.set_z(reg_val == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(false);
            },
            7 => {
                let u = reg_val >> 1;
                self.set_reg_8(bus, reg, u);
                self.set_z(u == 0);
                self.set_n(false);
                self.set_h(false);
                self.set_c(reg_val & 1 != 0);
            },
            _ => unreachable!(),
        }

    }

    pub fn push(&mut self, bus: &mut Bus) {

        let hi = ((self.read(self.cpu_ctx.instruction.register_1) >> 8) & 0xFF) as u8;
        self.emu_cycles(1);
        self.stack_push(bus, hi);

        let lo = (self.read(self.cpu_ctx.instruction.register_1) & 0xFF) as u8;
        self.emu_cycles(1);
        self.stack_push(bus, lo);

        self.emu_cycles(1);

    }

    pub fn pop(&mut self, bus: &mut Bus) {

        let hi = self.stack_pop(bus);
        self.emu_cycles(1);
        self.stack_push(bus, hi);

        let lo = self.stack_pop(bus);
        self.emu_cycles(1);
        self.stack_push(bus, lo);
    }

    pub fn add(&mut self, _bus: &mut Bus) {

        let mut val: u32 = self.read(self.cpu_ctx.instruction.register_1) as u32 + self.cpu_ctx.fetched_data as u32;

        let is_16bit: bool = is_16_bit(self.cpu_ctx.instruction.register_1);

        if is_16bit {
            self.emu_cycles(1);
        }

        if let RegisterType::SP = self.cpu_ctx.instruction.register_1 {
            val = self.read(self.cpu_ctx.instruction.register_1) as u32 + self.cpu_ctx.fetched_data as i8 as u32;
        }

        let mut z = (val & 0xFF) == 0; 
        let mut h = (self.read(self.cpu_ctx.instruction.register_1) & 0xF) + (self.cpu_ctx.fetched_data & 0xF) >= 0x10;
        let mut c = (self.read(self.cpu_ctx.instruction.register_1) as u32 & 0xF) + (self.cpu_ctx.fetched_data as u32 & 0xFF) >= 0x100; 

        if is_16bit {
            z = false;
            h = (self.read(self.cpu_ctx.instruction.register_1) & 0xFFF) + (self.cpu_ctx.fetched_data & 0xFFF) >= 0x1000;
            let n = (self.read(self.cpu_ctx.instruction.register_1) as u32) + self.cpu_ctx.fetched_data as u32;
            c = n >= 0x10000
        }

        if let RegisterType::SP = self.cpu_ctx.instruction.register_1 {
            z = false;
            h = (self.read(self.cpu_ctx.instruction.register_1) & 0xF) + (self.cpu_ctx.fetched_data & 0xF) > 0x100;
            c = (self.read(self.cpu_ctx.instruction.register_1) as u32 & 0xFF) + (self.cpu_ctx.fetched_data as u32 & 0xFF) > 0x100;
        }

        self.set_reg(self.cpu_ctx.instruction.register_1, (val & 0xFFFF) as u16);
        self.set_z(z);
        self.set_n(false);
        self.set_h(h);
        self.set_c(c);
    }

    pub fn adc(&mut self, _bus: &mut Bus) {
        
        let u = self.cpu_ctx.fetched_data;
        let a = self.a;
        let c_flag = self.get_c();

        let sum = a as u16 + u + (if c_flag {1} else {0});
        self.a = sum as u8;
        
        self.set_z(self.a == 0);
        self.set_n(false);
        self.set_h((a & 0xF) + (u & 0xF) as u8 + (if c_flag {1} else {0}) > 0xF);
        self.set_c(sum > 0xFF);

    }

    pub fn sub(&mut self, _bus: &mut Bus) {
        let reg_value = self.read(self.cpu_ctx.instruction.register_1);
        let fetched_value = self.cpu_ctx.fetched_data;

        let val = reg_value.wrapping_sub(fetched_value);

        let z = val == 0;
        let h = ((reg_value & 0xF) as i16).wrapping_sub((fetched_value & 0xF) as i16) < 0;
        let c = (reg_value as i16).wrapping_sub(fetched_value as i16) < 0;

        self.set_z(z);
        self.set_n(true);
        self.set_h(h);
        self.set_c(c);
    }

    // TODO: FIX THIS 
    pub fn sbc(&mut self, _bus: &mut Bus) {

        let old_carry_flag = if self.get_c() {1} else {0};
        let val: u8 = (self.cpu_ctx.fetched_data as u8).wrapping_add(old_carry_flag);

        let z = (self.read(self.cpu_ctx.instruction.register_1) as u8).wrapping_sub(val) == 0;
        let h = ((self.read(self.cpu_ctx.instruction.register_1) & 0xF) as i16)
            .wrapping_sub((self.cpu_ctx.fetched_data & 0xF) as i16)
            .wrapping_sub(old_carry_flag as i16) < 0;

        let c = (self.read(self.cpu_ctx.instruction.register_1) as i16)
            .wrapping_sub(self.cpu_ctx.fetched_data as i16)
            .wrapping_sub(old_carry_flag as i16) < 0;

        self.set_z(z);
        self.set_n(true);
        self.set_h(h);
        self.set_c(c);
    } 

    pub fn and(&mut self, _bus: &mut Bus) {

        self.a &= self.cpu_ctx.fetched_data as u8;
        self.set_z(self.a == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    }

    pub fn or(&mut self, _bus: &mut Bus) {
    
        self.a |= (self.cpu_ctx.fetched_data & 0xFF) as u8;
        self.set_z(self.a == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);

    }
 
    pub fn xor_8(&mut self, _bus: &mut Bus) {
        self.a ^= self.cpu_ctx.fetched_data as u8;
        self.set_z(self.a == 0);
        self.set_n(false);
        self.set_h(false);
        self.set_c(false);
    } 

    pub fn cp(&mut self, _bus: &mut Bus) {

        let n = (self.a as i16).wrapping_sub(self.cpu_ctx.fetched_data as i16);
        
        self.set_z(n == 0);
        self.set_n(true);
        self.set_h( (self.a as i16 & 0x0F).wrapping_sub(self.cpu_ctx.fetched_data as i16 & 0x0F) < 0 );
        self.set_c(n < 0);

    }
    
    pub fn inc(&mut self, bus: &mut Bus) {
        
        let mut val = self.read(self.cpu_ctx.instruction.register_1) + 1;

        if is_16_bit(self.cpu_ctx.instruction.register_1) {
            self.emu_cycles(1);
        }

        if let RegisterType::HL = self.cpu_ctx.instruction.register_1 {
            if let AddressMode::Mr = self.cpu_ctx.instruction.mode {
                let address = self.read(RegisterType::HL);
                val = bus.read(address, *self) as u16 + 1;
                val &= 0xFF;
                bus.write(self.read(RegisterType::HL), val as u8, self);
            } 
        }
        else {
            self.set_reg(self.cpu_ctx.instruction.register_1, val);
            val = self.read(self.cpu_ctx.instruction.register_1);
        }

        if self.cpu_ctx.current_opcode & 0x03 == 0x03 {
            return;
        }

        self.set_z(val == 0);
        self.set_n(false);
        self.set_h((val & 0x0F) == 0);

    }

    pub fn ldh(&mut self, bus: &mut Bus) {
        
        if let RegisterType::A = self.cpu_ctx.instruction.register_1{
            self.set_reg(self.cpu_ctx.instruction.register_1, bus.read(0xFF00 & self.cpu_ctx.fetched_data, *self) as u16);
        }
        else {
            bus.write(self.cpu_ctx.mem_dest, self.a, self);
        }

        self.emu_cycles(1);
    }
 
    pub fn dec(&mut self, bus: &mut Bus) {
        
        let reg_value = self.read(self.cpu_ctx.instruction.register_1);
        let mut val = reg_value.wrapping_sub(1);
    
        if is_16_bit(self.cpu_ctx.instruction.register_1) {
            self.emu_cycles(1);
        }

        if let RegisterType::HL = self.cpu_ctx.instruction.register_1 {
            if let AddressMode::Mr = self.cpu_ctx.instruction.mode {
                let address = self.read(RegisterType::HL);
                let current_value = bus.read(address, *self);
                let new_value = current_value.wrapping_sub(1); 
                bus.write(address, new_value, self);
            }
        }
        else {
            self.set_reg(self.cpu_ctx.instruction.register_1, val);
            val = self.read(self.cpu_ctx.instruction.register_1);
        }

        if self.cpu_ctx.current_opcode & 0x0B == 0x0B {
            return;
        }

        self.set_z(val == 0);
        self.set_n(true);
        self.set_h((val & 0x0F) == 0x0F);
    } 

    pub fn daa(&mut self, _bus: &mut Bus) {

        let a = self.a;

        if !self.get_n() {
            if self.get_c() || a > 0x99 {
                self.a = a.wrapping_add(0x60);
                self.set_c(true);
            }
            if self.get_h() || (a & 0xF) > 0x09 {
                self.a = self.a.wrapping_sub(0x06);
            }
        }
        else {
            if self.get_c() {
                self.a = self.a.wrapping_sub(0x60)
            }
            if self.get_h() {
                self.a = self.a.wrapping_sub(0x06);  
            }
        }

        self.set_z(self.a == 0);
        self.set_h(false);
    }

    pub fn cpl(&mut self, _bus: &mut Bus) {
        self.a = !self.a;

        self.set_n(true); 
        self.set_h(true);
    }

    pub fn ccf(&mut self, _bus: &mut Bus) {
        let c_flag = self.get_c();
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(c_flag ^ true); 
    }

    pub fn scf(&mut self, _bus: &mut Bus) {
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(true); 
    }

    pub fn nop(&mut self, _bus: &mut Bus) {
        
    }

    pub fn halt(&mut self, _bus: &mut Bus) {
        self.cpu_ctx.halted = true;
    }

    pub fn stop(&mut self, _bus: &mut Bus) {

        println!("Not Done: stop");
        self.cpu_ctx.halted = true;

    }
    
    pub fn di(&mut self, _bus: &mut Bus) {
        self.cpu_ctx.int_master_enabled = false;
    }

    pub fn ei(&mut self, _bus: &mut Bus) {

        self.cpu_ctx.enable_ime = true;

    }

    pub fn rlca(&mut self, _bus: &mut Bus) {
        
        let a = self.a;   
        let c = ((a >> 7) & 1) != 0;
        let a = (a << 1) | if c {1} else {0};
        self.a = a;

        self.set_z(false); 
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(c); 

    }

    pub fn rla(&mut self, _bus: &mut Bus) {
        
        let copy_of_a = self.a;
        let c_flag = self.get_c();
        let new_c_flag = (copy_of_a >> 7) & 1 != 0; 
        
        self.a = (copy_of_a << 1) | if c_flag {1} else {0};
        
        self.set_z(false); 
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(new_c_flag); 
    }

    pub fn rrca(&mut self, _bus: &mut Bus) {
        
        let lo_a = self.a & 1;
        
        self.a >>= 1;
        self.a |= lo_a << 7;

        self.set_z(false); 
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(lo_a != 0); 
    }

    pub fn rra(&mut self, _bus: &mut Bus) {
        
        let c_flag = self.get_c();
        let new_c_flag = (self.a & 1) == 1;

        self.a >>= 1;
        if c_flag {
            self.a |= 0b1000_0000;
        }
        else {
            self.a &= 0b0111_1111;
        }

        self.set_z(false); 
        self.set_n(false); 
        self.set_h(false); 
        self.set_c(new_c_flag); 
    }

    pub fn jp(&mut self, bus: &mut Bus) {
        self.goto_address(bus, self.cpu_ctx.fetched_data, false)
    }

    pub fn jr(&mut self, bus: &mut Bus) {
        let relitive_jump_amount = self.cpu_ctx.fetched_data  as i8;
        let address = self.pc.wrapping_add(relitive_jump_amount as u16);
        self.goto_address(bus, address, false)
    }

    pub fn call(&mut self, bus: &mut Bus) {
        self.goto_address(bus, self.cpu_ctx.fetched_data, true)
    }

    pub fn rst(&mut self, bus: &mut Bus) {
        self.goto_address(bus, self.cpu_ctx.instruction.parmater.into() , true)
    }

    pub fn ret(&mut self, bus: &mut Bus) {
    
        if let ConditionType::None = self.cpu_ctx.instruction.condition {
            self.emu_cycles(1)
        }

        if self.check_cond() {
            let lo: u16 = self.stack_pop(bus).into();
            self.emu_cycles(1);

            let hi: u16 = self.stack_pop(bus).into();
            self.emu_cycles(1);

            let n: u16 = (hi << 8 ) | lo;
            self.pc = n;
            self.emu_cycles(1);
        }

    }

    pub fn reti(&mut self, bus: &mut Bus) {

        self.cpu_ctx.int_master_enabled = true;
        self.ret(bus);

    }
    
    // misc helper functions
    
    fn goto_address(&mut self, bus: &mut Bus, address: u16, pushpc: bool) {
        if self.check_cond() && pushpc {
            self.emu_cycles(2);
            self.stack_push16(bus, self.pc);
        }

        self.pc = address;
        self.emu_cycles(1);
    }
}
