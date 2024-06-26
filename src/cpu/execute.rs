use crate::cpu::Register16;
use crate::cpu::Register8;
use crate::cpu::decode::Condition;
use crate::cpu::Cpu;

impl Cpu {
    pub fn load_8<O: Copy, I: Copy>(&mut self, out8: O, in8: I) {

    }

    pub fn push_16(&mut self, reg: Register16) {

    }

    pub fn pop_16(&mut self, reg: Register16) {

    }

    pub fn load_16_immediate(&mut self, reg: Register16) {

    }

    pub fn load_16_sp_nn(&mut self) {

    }

    pub fn load_16_nn_sp(&mut self) {

    }

    pub fn load_sp_hl(&mut self) {

    }

    pub fn add_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn adc_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn sub_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn sbc_8<I: Copy>(&mut self, in8: I) {

    } 

    pub fn and_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn or_8<I: Copy>(&mut self, in8: I) {

    }
    
    pub fn xor_8<I: Copy>(&mut self, in8: I) {

    } 

    pub fn cp_8<I: Copy>(&mut self, in8: I) {

    }
    
    pub fn inc_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn dec_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn add_HL_16<I: Copy>(&mut self, in16: I) {

    }
    
    pub fn add_sp_16(&mut self) {
        
    }

    pub fn inc_16<I: Copy>(&mut self, in16: I) {
    
    }

    pub fn dec_16<I: Copy>(&mut self, in16: I) {

    } 

    pub fn swap_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn daa(&mut self) {

    }

    pub fn srl_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn cpl(&mut self) {

    }

    pub fn ccf(&mut self) {

    }

    pub fn scf(&mut self) {

    }

    pub fn nop(&mut self) {

    }

    pub fn halt(&mut self) {

    }

    pub fn stop(&mut self) {

    }
    
    pub fn di(&mut self) {

    }

    pub fn ei(&mut self) {

    }

    pub fn rlca(&mut self) {

    }

    pub fn rla(&mut self) {

    }

    pub fn rrca(&mut self) {
        
    }

    pub fn rra(&mut self) {

    }

    pub fn rlc_8<I: Copy>(&mut self, in8: I) {
    
    }

    pub fn rl_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn rrc_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn rr_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn sla_8 <I: Copy>(&mut self, in8: I ) {

    }

    pub fn sra_8 <I: Copy>(&mut self, in8: I ) {

    }

    pub fn slr_8 <I: Copy>(&mut self, in8: I ) {

    }

    pub fn jp(&mut self) {
    }

    pub fn jp_cc (&mut self, con: Condition) {

    }

    pub fn jp_hl (&mut self) {

    }

    pub fn jr (&mut self) {

    }

    pub fn jr_cc(&mut self, con: Condition) {

    }

    pub fn call(&mut self) {

    }

    pub fn call_cc(&mut self, con: Condition) {

    }

    pub fn rst(&mut self, addr: u8) {

    }

    pub fn ret(&mut self) {

    }

    pub fn ret_cc(&mut self, con: Condition) {

    }

    pub fn reti(&mut self) {

    }

    pub fn bit<I: Copy>(&mut self, b: I) {

    }

    pub fn set<I: Copy>(&mut self, b: I) {

    }

    pub fn res<I: Copy>(&mut self, b: I) {

    }

} 
