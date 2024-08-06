use crate::cpu::RegisterType;
use crate::cpu::structs::AddressMode;
use crate::cpu::Cpu;

impl Cpu {

    pub fn load_8(&mut self) {

        println!("Not Done: load_8");
        self.cpu_ctx.halted = true;

    }

    pub fn push_16(&mut self) {

        println!("Not Done: push_16");
        self.cpu_ctx.halted = true;

    }

    pub fn pop_16(&mut self) {

        println!("Not Done: pop_16");
        self.cpu_ctx.halted = true;

    }

    pub fn load_16_immediate(&mut self) {

        println!("Not Done: load_16_immediate");
        self.cpu_ctx.halted = true;
    }

    pub fn load_16_sp_nn(&mut self) {

        println!("Not Done: load_16_sp_nn");
        self.cpu_ctx.halted = true;
    
    }

    pub fn load_16_nn_sp(&mut self) {

        println!("Not Done: load_16_nn_sp");
        self.cpu_ctx.halted = true;

    }

    pub fn load_sp_hl(&mut self) {

        println!("Not Done: load_sp_hl");
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
 
    pub fn xor_8(&mut self) {
        self.registers.a ^= self.cpu_ctx.fetched_data as u8;
        self.registers.set_flags(self.registers.a == 0, false, false, false);
    } 

    pub fn cp_8(&mut self) {

        println!("Not Done: cp_8");
        self.cpu_ctx.halted = true;

    }
    
    pub fn inc_8(&mut self) {

        println!("Not Done: inc_8");
        self.cpu_ctx.halted = true;

    }

    pub fn dec_8(&mut self) {

        println!("Not Done: dec_8");
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

    pub fn dec_16(&mut self) {

        println!("Not Done: dec_16");
        self.cpu_ctx.halted = true;

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

    pub fn ccf(&mut self) {

        println!("Not Done: ccf");
        self.cpu_ctx.halted = true;

    }

    pub fn scf(&mut self) {

        println!("Not Done: scf");
        self.cpu_ctx.halted = true;

    }

    pub fn nop(&mut self) {
        
    }

    pub fn halt(&mut self) {

        println!("Not Done: halt");
        self.cpu_ctx.halted = true;

    }

    pub fn stop(&mut self) {

        println!("Not Done: stop");
        self.cpu_ctx.halted = true;

    }
    
    pub fn di(&mut self) {

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

    pub fn rla(&mut self) {

        println!("Not Done: rla");
        self.cpu_ctx.halted = true;

    }

    pub fn rrca(&mut self) {

        println!("Not Done: rrca");
        self.cpu_ctx.halted = true;
        
    }

    pub fn rra(&mut self) {

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

    pub fn jp(&mut self) {
        
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
