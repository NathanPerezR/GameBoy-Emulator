use crate::cpu::Register16;
use crate::cpu::Register8;
use crate::cpu::decode::Condition;
use crate::cpu::Cpu;
use crate::cpu::Step;

impl Cpu {

    pub fn load_8<O: Copy, I: Copy>(&mut self, out8: O, in8: I) -> Step {
        self.prefetch_next(self.registers.pc)
    }

    pub fn push_16(&mut self, reg: Register16) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn pop_16(&mut self, reg: Register16) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn load_16_immediate(&mut self, reg: Register16) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn load_16_sp_nn(&mut self) -> Step {
    
        self.prefetch_next(self.registers.pc)
    }

    pub fn load_16_nn_sp(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn load_sp_hl(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn add_8<I: Copy>(&mut self, in8: I) -> Step {
        
        self.prefetch_next(self.registers.pc)
    }

    pub fn adc_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn sub_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn sbc_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    } 

    pub fn and_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn or_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }
    
    pub fn xor_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    } 

    pub fn cp_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }
    
    pub fn inc_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn dec_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn add_HL_16<I: Copy>(&mut self, in16: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }
    
    pub fn add_sp_16(&mut self) -> Step {
        
        self.prefetch_next(self.registers.pc)
    }

    pub fn inc_16<I: Copy>(&mut self, in16: I) -> Step {
    
        self.prefetch_next(self.registers.pc)
    }

    pub fn dec_16<I: Copy>(&mut self, in16: I) -> Step {

        self.prefetch_next(self.registers.pc)
    } 

    pub fn swap_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn daa(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn srl_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn cpl(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn ccf(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn scf(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn nop(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn halt(&mut self) -> Step {

    }

    pub fn stop(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }
    
    pub fn di(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn ei(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rlca(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rla(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rrca(&mut self) -> Step {
        
        self.prefetch_next(self.registers.pc)
    }

    pub fn rra(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rlc_8<I: Copy>(&mut self, in8: I) -> Step {
    
        self.prefetch_next(self.registers.pc)
    }

    pub fn rl_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rrc_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rr_8<I: Copy>(&mut self, in8: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn sla_8 <I: Copy>(&mut self, in8: I ) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn sra_8 <I: Copy>(&mut self, in8: I ) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn slr_8 <I: Copy>(&mut self, in8: I ) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn jp(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn jp_cc (&mut self, con: Condition) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn jp_hl (&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn jr (&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn jr_cc(&mut self, con: Condition) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn call(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn call_cc(&mut self, con: Condition) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn rst(&mut self, addr: u8) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn ret(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn ret_cc(&mut self, con: Condition) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn reti(&mut self) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn bit<I: Copy>(&mut self, b: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn set<I: Copy>(&mut self, b: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

    pub fn res<I: Copy>(&mut self, b: I) -> Step {

        self.prefetch_next(self.registers.pc)
    }

} 
