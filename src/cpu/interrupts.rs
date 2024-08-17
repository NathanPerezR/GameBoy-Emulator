use crate::cpu::Cpu;
use crate::bus::Bus;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum InterruptType {
    VbBlank = 1,
    LcdStat = 2,
    Timer = 4,
    Serial = 8,
    Joypad = 16,
}

impl Cpu {

    fn interrupt_handle(&mut self, bus: &mut Bus , address: u16) {
        self.stack_push16(bus, self.pc);
        self.pc = address;
    }


    fn interrupt_check(&mut self, bus: &mut Bus, address: u16, interrupt: InterruptType ) -> bool {
        
        if (self.cpu_ctx.interrupt_flag & (interrupt as u8)) != 0 && (self.cpu_ctx.ie_register & (interrupt as u8)) != 0 {
            self.interrupt_handle(bus, address);     
            self.cpu_ctx.interrupt_flag &= !(interrupt as u8);
            self.cpu_ctx.halted = false;
            self.cpu_ctx.int_master_enabled = false;
            
            return true;
        }

        false
    }
    
    pub fn handle_interrupt(&mut self, bus: &mut Bus) {
        if self.interrupt_check(bus, 0x40, InterruptType::VbBlank) {
 
        }
        else if self.interrupt_check(bus, 0x48, InterruptType::LcdStat) {

        }
        else if self.interrupt_check(bus, 0x50, InterruptType::Timer) {

        }
        else if self.interrupt_check(bus, 0x58, InterruptType::Serial) {

        }
        else if self.interrupt_check(bus, 0x60, InterruptType::Joypad) {

        }
    }

}
