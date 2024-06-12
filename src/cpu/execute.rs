use crate::cpu::Register16;
use crate::cpu::Register8;
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

    pub fn load_16_sp_nn() {

    }

    pub fn load_16_nn_sp() {

    }

    pub fn load_sp_hl() {

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
    
    pub fn add_SP_16<I: Copy>(&mut self) {
        
    }

    pub fn inc_16<I: Copy>(&mut self, in16: I) {
    
    }

    pub fn dec_16<I: Copy>(&mut self, in16: I) {

    } 

    pub fn swap_8<I: Copy>(&mut self, in8: I) {

    }

    pub fn daa(&mut self) {

    }

    pub fn cpl(&mut self) {

    }

    pub fn ccf(&mut self) {

    }

    pub fn scf(&mut self) {

    }

    pub fn nop(&mut self) {

    }
} 
