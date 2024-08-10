use crate::cpu::{Cpu, AddressMode, register::RegisterType};
use crate::bus::Bus;

impl Cpu {
    // fetching data depends on the current mode of the CPU
    pub fn fetch_data(&mut self, bus: &mut Bus) {
        self.cpu_ctx.mem_dest = 0;
        self.cpu_ctx.dest_is_mem = false;
        
        use AddressMode::*;
        match &self.cpu_ctx.instruction.mode {
            Imp => (),                                
            R => {                                                
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_1);
            },
            RR => {
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_1);
            }
            RD8 => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc = self.registers.pc.wrapping_add(1);
            },
            RD16 | D16 => {
                let lo: u8 = bus.read(self.registers.pc);
                self.emu_cycles(1);
                                                                                                          
                let hi: u8 = bus.read(self.registers.pc + 1);
                self.emu_cycles(1);
                self.registers.pc = self.registers.pc.wrapping_add(2);
                                                                                                          
                self.cpu_ctx.fetched_data = ((hi as u16) << 8) | (lo as u16); 
            },
            MrR => {
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                
                use RegisterType::C;
                if let C = self.cpu_ctx.instruction.register_1 {self.cpu_ctx.mem_dest |= 0xFF00}
            },
            RMr => {
                let mut address: u16 = self.registers.read(self.cpu_ctx.instruction.register_2);

                if let RegisterType::C = self.cpu_ctx.instruction.register_2 {address |= 0xFF00}

                self.cpu_ctx.fetched_data = bus.read(address) as u16;
                self.emu_cycles(1);
            }
            RHli => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.read(self.cpu_ctx.instruction.register_2)) as u16;
                self.emu_cycles(1);
                self.registers.set_reg(RegisterType::HL, self.registers.read(RegisterType::HL).wrapping_add(1))
                
            },
            RHld => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.read(self.cpu_ctx.instruction.register_2)) as u16;
                self.emu_cycles(1);
                self.registers.set_reg(RegisterType::HL, self.registers.read(RegisterType::HL).wrapping_sub(1));
            },
            HliR => {
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.registers.set_reg(RegisterType::HL, self.registers.read(RegisterType::HL).wrapping_add(1));
            },
            HldR => {
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.registers.set_reg(RegisterType::HL, self.registers.read(RegisterType::HL).wrapping_sub(1))
            },
            RA8 => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            }, 
            A8R => {
                self.cpu_ctx.mem_dest = bus.read(self.registers.pc) as u16;
                self.cpu_ctx.dest_is_mem = true;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            HlSpr => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            D8 => {
                self.cpu_ctx.fetched_data = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            A16R | D16R => {   
                let lo: u16 = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);

                let hi: u16 = bus.read(self.registers.pc + 1) as u16;
                self.emu_cycles(1);

                self.cpu_ctx.mem_dest = lo | (hi << 8);
                self.cpu_ctx.dest_is_mem = true;

                self.registers.pc += 2;
                self.cpu_ctx.fetched_data = self.registers.read(self.cpu_ctx.instruction.register_2);
            },
            Mr => {
                self.cpu_ctx.mem_dest = self.registers.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.cpu_ctx.fetched_data = bus.read(self.registers.read(self.cpu_ctx.instruction.register_1)) as u16;
            },
            RA16 => {
                let lo: u16 = bus.read(self.registers.pc) as u16;
                self.emu_cycles(1);

                let hi: u16 = bus.read(self.registers.pc + 1) as u16;
                self.emu_cycles(1);

                let address: u16 = lo | (hi << 8);
                self.cpu_ctx.dest_is_mem = true;

                self.registers.pc += 2;
                self.cpu_ctx.fetched_data = bus.read(address) as u16;
                self.emu_cycles(1);
            },
            _ => { 
                println!("Unknown Addressing mode hit");
            }
        }
    }
}

