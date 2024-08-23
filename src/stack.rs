use crate::cpu::Cpu;
use crate::bus::Bus;

impl Cpu {
    
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

    // pub fn stack_pop16(&mut self, bus: &mut Bus) -> u16 {
    //     let lo: u16 = self.stack_pop(bus).into();
    //     let hi: u16 = self.stack_pop(bus).into();
    // 
    //     (lo) | (hi << 8)
    // }


}
