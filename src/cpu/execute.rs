use crate::cpu::RegisterType;
use crate::cpu::register::is_16_bit;
use crate::cpu::structs::AddressMode;
use crate::cpu::Cpu;
use crate::cart::Cart;
use crate::bus::{bus_write, bus_read, bus_write16};

impl Cpu {

    pub fn load(&mut self, cart: &mut Cart) {
        if self.cpu_ctx.dest_is_mem {

            use RegisterType::*;
            match self.cpu_ctx.instruction.register_2 {
                A | B | C | D | E | L | F | H => {
                    bus_write(cart, self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data as u8);   
                },
                AF | BC | DE | HL | PC | SP => {
                    self.emu_cycles(1);
                    bus_write16(cart, self.cpu_ctx.mem_dest, self.cpu_ctx.fetched_data);
                },
                _ => println!("Error, none selected")
            }
            return;
        }

        if let AddressMode::HlSpr = self.cpu_ctx.instruction.mode {
            let h_flag: bool = (self.registers.read_reg(self.cpu_ctx.instruction.register_2) & 0xF) + 
                (self.cpu_ctx.fetched_data & 0xF) >= 0x10;
            let c_flag: bool = (self.registers.read_reg(self.cpu_ctx.instruction.register_2) & 0xFF) +
                (self.cpu_ctx.fetched_data & 0xFF) >= 0x100;
            
            self.registers.set_z(false);
            self.registers.set_n(false);
            self.registers.set_h(h_flag);
            self.registers.set_c(c_flag);

            let value_reg_2 = self.registers.read_reg(self.cpu_ctx.instruction.register_2);
            let new_value = value_reg_2.wrapping_add(self.cpu_ctx.fetched_data);
            self.registers.set_reg(self.cpu_ctx.instruction.register_1, new_value);
            return;
        }

        self.registers.set_reg(self.cpu_ctx.instruction.register_1, self.cpu_ctx.fetched_data)
    }

    pub fn push_16(&mut self) {

        println!("Not Done: push_16");
        self.cpu_ctx.halted = true;

    }

    pub fn pop_16(&mut self) {

        println!("Not Done: pop_16");
        self.cpu_ctx.halted = true;

    }

    pub fn add_8(&mut self) {


        println!("Not Done: add_8");
        self.cpu_ctx.halted = true;
        
    }

    pub fn adc_8(&mut self) {

        println!("Not Done: adc_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sub_8(&mut self) {

        println!("Not Done: sub_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sbc_8(&mut self) {

        println!("Not Done: sbc_8");
        self.cpu_ctx.halted = true;

    } 

    pub fn and_8(&mut self) {

        println!("Not Done: and_8");
        self.cpu_ctx.halted = true;

    }

    pub fn or_8(&mut self) {

        println!("Not Done: or_8");
        self.cpu_ctx.halted = true;

    }
 
    pub fn xor_8(&mut self, _cart: &mut Cart) {
        self.registers.a ^= self.cpu_ctx.fetched_data as u8;
        self.registers.set_z(self.registers.read_reg(RegisterType::A) == 0);
        self.registers.set_n(false);
        self.registers.set_h(false);
        self.registers.set_c(false);
    } 

    pub fn cp_8(&mut self) {

        println!("Not Done: cp_8");
        self.cpu_ctx.halted = true;

    }
    
    pub fn inc_8(&mut self) {

        println!("Not Done: inc_8");
        self.cpu_ctx.halted = true;

    }

    pub fn add_hl_16(&mut self) {

        println!("Not Done: add_hl_16");
        self.cpu_ctx.halted = true;

    }
    
    pub fn add_sp_16(&mut self) {

        println!("Not Done: add_sp_16");
        self.cpu_ctx.halted = true;
        
    }

    pub fn inc_16(&mut self) {

        println!("Not Done: inc_16");
        self.cpu_ctx.halted = true;
    
    }

    pub fn dec(&mut self, cart: &mut Cart) {
        
        let reg_value = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
        let mut val = reg_value.wrapping_sub(1);
    
        if is_16_bit(self.cpu_ctx.instruction.register_1) {
            self.emu_cycles(1);
        }

        if let RegisterType::HL = self.cpu_ctx.instruction.register_1 {
            if let AddressMode::Mr = self.cpu_ctx.instruction.mode {
                let address = self.registers.read_reg(RegisterType::HL);
                let current_value = bus_read(cart, address);
                let new_value = current_value.wrapping_sub(1); 
                bus_write(cart, address, new_value);
            }
        }
        else {
            self.registers.set_reg(self.cpu_ctx.instruction.register_1, val);
            val = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
        }

        if self.cpu_ctx.current_opcode & 0x0B == 0x0B {
            return;
        }

        self.registers.set_z(val == 0);
        self.registers.set_n(true);
        self.registers.set_h((val & 0x0F) == 0x0F);
    } 

    pub fn swap_8(&mut self) {

        println!("Not Done: swap_8");
        self.cpu_ctx.halted = true;

    }

    pub fn daa(&mut self) {

        println!("Not Done: daa");
        self.cpu_ctx.halted = true;

    }

    pub fn srl_8(&mut self) {

        println!("Not Done: srl_8");
        self.cpu_ctx.halted = true;

    }

    pub fn cpl(&mut self) {

        println!("Not Done: cpl");
        self.cpu_ctx.halted = true;

    }

    pub fn ccf(&mut self, _cart: &mut Cart) {
        let c_flag = self.registers.get_c();
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(c_flag ^ true); 
    }

    pub fn scf(&mut self) {

        println!("Not Done: scf");
        self.cpu_ctx.halted = true;

    }

    pub fn nop(&mut self, _cart: &mut Cart) {
        
    }

    pub fn halt(&mut self, _cart: &mut Cart) {
        self.cpu_ctx.halted = true;
    }

    pub fn stop(&mut self) {

        println!("Not Done: stop");
        self.cpu_ctx.halted = true;

    }
    
    pub fn di(&mut self, _cart: &mut Cart) {
        self.cpu_ctx.int_master_enabled = false;

    }

    pub fn ei(&mut self) {

        println!("Not Done: ei");
        self.cpu_ctx.halted = true;

    }

    pub fn rlca(&mut self) {

        println!("Not Done: rlca");
        self.cpu_ctx.halted = true;

    }

    // TODO: TEST THIS
    pub fn rla(&mut self, _cart: &mut Cart) {
        
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

    // TODO: TEST THIS
    pub fn rrca(&mut self, _cart: &mut Cart) {
        
        let lo_a = self.registers.a & 1;
        
        self.registers.a >>= 1;
        self.registers.a |= lo_a << 7;

        self.registers.set_z(false); 
        self.registers.set_n(false); 
        self.registers.set_h(false); 
        self.registers.set_c(lo_a != 0); 
    }

    // TODO: TEST THIS
    pub fn rra(&mut self, _cart: &mut Cart) {
        
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

    pub fn rlc_8(&mut self) {

        println!("Not Done: rlc_8");
        self.cpu_ctx.halted = true;
    
    }


    pub fn rl_8(&mut self) {

        println!("Not Done: rl_8");
        self.cpu_ctx.halted = true;

    }

    pub fn rrc_8(&mut self) {

        println!("Not Done: rrc_8");
        self.cpu_ctx.halted = true;

    }

    pub fn rr_8(&mut self) {

        println!("Not Done: rr_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sla_8 (&mut self ) {

        println!("Not Done: sla_8");
        self.cpu_ctx.halted = true;

    }

    pub fn sra_8 (&mut self ) {

        println!("Not Done: sra_8");
        self.cpu_ctx.halted = true;

    }

    pub fn slr_8 (&mut self ) {

        println!("Not Done: slr_8");
        self.cpu_ctx.halted = true;

    }

    pub fn jp(&mut self, _cart: &mut Cart) {
        
        if self.check_cond() {
            self.registers.pc = self.cpu_ctx.fetched_data;
            self.emu_cycles(1);
        } 
    }

    pub fn jp_cc (&mut self) {

        println!("Not Done: jp_cc");
        self.cpu_ctx.halted = true;

    }

    pub fn jp_hl (&mut self) {

        println!("Not Done: jp_hl");
        self.cpu_ctx.halted = true;

    }

    pub fn jr (&mut self) {

        println!("Not Done: jr");
        self.cpu_ctx.halted = true;

    }

    pub fn jr_cc(&mut self) {

        println!("Not Done: jr_cc");
        self.cpu_ctx.halted = true;

    }

    pub fn call(&mut self) {

        println!("Not Done: call");
        self.cpu_ctx.halted = true;

    }

    pub fn call_cc(&mut self) {

        println!("Not Done: call_cc");
        self.cpu_ctx.halted = true;

    }

    pub fn rst(&mut self) {

        println!("Not Done: rst");
        self.cpu_ctx.halted = true;

    }

    pub fn ret(&mut self) {

        println!("Not Done: ret");
        self.cpu_ctx.halted = true;

    }

    pub fn ret_cc(&mut self) {

        println!("Not Done: ret_cc");
        self.cpu_ctx.halted = true;

    }

    pub fn reti(&mut self) {

        println!("Not Done: reti");
        self.cpu_ctx.halted = true;

    }

    pub fn bit(&mut self) {

        println!("Not Done: bit");
        self.cpu_ctx.halted = true;

    }

    pub fn set(&mut self) {

        println!("Not Done: set");
        self.cpu_ctx.halted = true;

    }

    pub fn res(&mut self,) {

        println!("Not Done: res");
        self.cpu_ctx.halted = true;
    
    }

} 
