use crate::bus::Bus;
use crate::cpu::{RegisterType, ConditionType};
use crate::cpu::register::is_16_bit;
use crate::cpu::structs::AddressMode;
use crate::cpu::Cpu;

impl Cpu {

    pub fn load(&mut self, bus: &mut Bus) {
        if self.cpu_ctx.dest_is_mem {

            use RegisterType::*;
            match self.cpu_ctx.instruction.register_2 {
                A | B | C | D | E | L | F | H => {
                    bus.write(self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data as u8);   
                },
                AF | BC | DE | HL | PC | SP => {
                    self.emu_cycles(1);
                    bus.write16(self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data);
                },
                _ => println!("Error, none selected")
            }
            return;
        }

        if let AddressMode::HlSpr = self.cpu_ctx.instruction.mode {
            let h_flag: bool = (self.registers.read(self.cpu_ctx.instruction.register_2) & 0xF).wrapping_add 
                (self.cpu_ctx.fetched_data & 0xF) >= 0x10;
            let c_flag: bool = (self.registers.read(self.cpu_ctx.instruction.register_2) & 0xFF).wrapping_add
                (self.cpu_ctx.fetched_data & 0xFF) >= 0x100;
            
            self.registers.set_z(false);
            self.registers.set_n(false);
            self.registers.set_h(h_flag);
            self.registers.set_c(c_flag);

            let value_reg_2 = self.registers.read(self.cpu_ctx.instruction.register_2);
            let new_value = value_reg_2.wrapping_add(self.cpu_ctx.fetched_data);
            self.registers.set_reg(self.cpu_ctx.instruction.register_1, new_value);
            return;
        }

        self.registers.set_reg(self.cpu_ctx.instruction.register_1, self.cpu_ctx.fetched_data)
    }

    pub fn push(&mut self, _bus: &mut Bus) {

        println!("Not Done: push");
        self.cpu_ctx.halted = true;

    }

    pub fn pop(&mut self, _bus: &mut Bus) {

        println!("Not Done: pop");
        self.cpu_ctx.halted = true;

    }

    pub fn add(&mut self, _bus: &mut Bus) {

        println!("Not Done: add");
        self.cpu_ctx.halted = true;
        
    }

    pub fn adc(&mut self, _bus: &mut Bus) {

        println!("Not Done: adc");
        self.cpu_ctx.halted = true;

    }

    pub fn sub(&mut self, _bus: &mut Bus) {


        println!("Not Done: sub");
        self.cpu_ctx.halted = true;

    }

    pub fn sbc(&mut self, _bus: &mut Bus) {

        println!("Not Done: sbc");
        self.cpu_ctx.halted = true;

    } 

    pub fn and(&mut self, _bus: &mut Bus) {

        println!("Not Done: and");
        self.cpu_ctx.halted = true;

    }

    pub fn or(&mut self, _bus: &mut Bus) {
 
        println!("Not Done: or");
        self.cpu_ctx.halted = true;

    }
 
    pub fn xor_8(&mut self, _bus: &mut Bus) {
        self.registers.a ^= self.cpu_ctx.fetched_data as u8;
        self.registers.set_z(self.registers.read(RegisterType::A) == 0);
        self.registers.set_n(false);
        self.registers.set_h(false);
        self.registers.set_c(false);
    } 

    pub fn cp(&mut self, _bus: &mut Bus) {

        println!("Not Done: cp");
        self.cpu_ctx.halted = true;

    }
    
    pub fn inc(&mut self, _bus: &mut Bus) {

        println!("Not Done: inc");
        self.cpu_ctx.halted = true;

    }

    pub fn ldh(&mut self, bus: &mut Bus) {
        
        if let RegisterType::A = self.cpu_ctx.instruction.register_1{
            self.registers.set_reg(self.cpu_ctx.instruction.register_1, bus.read(0xFF00 & self.cpu_ctx.fetched_data) as u16);
        }
        else {
            bus.write(self.cpu_ctx.mem_dest, self.registers.a)
        }

        self.emu_cycles(1);
    }
 
    pub fn dec(&mut self, bus: &mut Bus) {
        
        let reg_value = self.registers.read(self.cpu_ctx.instruction.register_1);
        let mut val = reg_value.wrapping_sub(1);
    
        if is_16_bit(self.cpu_ctx.instruction.register_1) {
            self.emu_cycles(1);
        }

        if let RegisterType::HL = self.cpu_ctx.instruction.register_1 {
            if let AddressMode::Mr = self.cpu_ctx.instruction.mode {
                let address = self.registers.read(RegisterType::HL);
                let current_value = bus.read(address);
                let new_value = current_value.wrapping_sub(1); 
                bus.write(address, new_value);
            }
        }
        else {
            self.registers.set_reg(self.cpu_ctx.instruction.register_1, val);
            val = self.registers.read(self.cpu_ctx.instruction.register_1);
        }

        if self.cpu_ctx.current_opcode & 0x0B == 0x0B {
            return;
        }

        self.registers.set_z(val == 0);
        self.registers.set_n(true);
        self.registers.set_h((val & 0x0F) == 0x0F);
    } 

    pub fn daa(&mut self, _bus: &mut Bus) {

        println!("Not Done: daa");
        self.cpu_ctx.halted = true;

    }

    pub fn cpl(&mut self, _bus: &mut Bus) {

        println!("Not Done: cpl");
        self.cpu_ctx.halted = true;

    }

    pub fn ccf(&mut self, _bus: &mut Bus) {
        let c_flag = self.registers.get_c();
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(c_flag ^ true); 
    }

    pub fn scf(&mut self, _bus: &mut Bus) {

        println!("Not Done: scf");
        self.cpu_ctx.halted = true;

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

        println!("Not Done: ei");
        self.cpu_ctx.halted = true;

    }

    pub fn rlca(&mut self, _bus: &mut Bus) {
        
        println!("Not Done: rlca");
        self.cpu_ctx.halted = true;

    }

    pub fn rla(&mut self, _bus: &mut Bus) {
        
        let copy_of_a = self.registers.a;
        let c_flag = self.registers.get_c();
        let new_c_flag = (copy_of_a >> 7) & 1 != 0; 
        
        self.registers.a = (copy_of_a << 1) | if c_flag {1} else {0};
        
        self.registers.set_z(false); 
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(new_c_flag); 
        println!("Not Done: rla");
        self.cpu_ctx.halted = true;

    }

    pub fn rrca(&mut self, _bus: &mut Bus) {
        
        let lo_a = self.registers.a & 1;
        
        self.registers.a >>= 1;
        self.registers.a |= lo_a << 7;

        self.registers.set_z(false); 
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(lo_a != 0); 
    }

    pub fn rra(&mut self, _bus: &mut Bus) {
        
        let c_flag = self.registers.get_c();
        let new_c_flag = (self.registers.a & 1) == 1;

        self.registers.a >>= 1;
        if c_flag {
            self.registers.a |= 0b1000_0000;
        }
        else {
            self.registers.a &= 0b0111_1111;
        }

        self.registers.set_z(false); 
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(new_c_flag); 

        println!("Not Done: rra");
        self.cpu_ctx.halted = true;

    }

    pub fn rlc(&mut self, _bus: &mut Bus) {

        println!("Not Done: rlc_8");
        self.cpu_ctx.halted = true;
    
    }


    pub fn rl(&mut self, _bus: &mut Bus) {

        println!("Not Done: rl_8");
        self.cpu_ctx.halted = true;

    }

    pub fn rrc(&mut self, _bus: &mut Bus) {

        println!("Not Done: rrc_8");
        self.cpu_ctx.halted = true;

    }

    pub fn rr(&mut self, _bus: &mut Bus) {

        println!("Not Done: rr_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sla(&mut self, _bus: &mut Bus) {

        println!("Not Done: sla_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sra(&mut self, _bus: &mut Bus) {

        println!("Not Done: sra_8");
        self.cpu_ctx.halted = true;

    }

    pub fn slr(&mut self, _bus: &mut Bus) {

        println!("Not Done: slr_8");
        self.cpu_ctx.halted = true;

    }


    pub fn jp(&mut self, bus: &mut Bus) {
        self.goto_address(bus, self.cpu_ctx.fetched_data, false)
    }

    pub fn jr(&mut self, bus: &mut Bus) {
        let relitive_jump_amount = self.cpu_ctx.fetched_data  as i8;
        let address = self.registers.pc.wrapping_add(relitive_jump_amount as u16);
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
            self.registers.pc = n;
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
            self.stack_push16(bus, self.registers.pc);
        }

        self.registers.pc = address;
        self.emu_cycles(1);
    }
}
