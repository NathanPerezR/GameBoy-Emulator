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
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_1);
            },
            RR => {
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_2);
            }
            RD8 => {
                self.cpu_ctx.fetched_data = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);
                self.pc = self.pc.wrapping_add(1);
            },
            RD16 | D16 => {
                let lo: u16 = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);
                                                                                                          
                let hi: u16 = bus.read(self.pc.wrapping_add(1), self) as u16;
                self.emu_cycles(1, bus);
                                                                                                     
                self.cpu_ctx.fetched_data = lo | (hi << 8); 

                self.pc = self.pc.wrapping_add(2);

            },
            MrR => {
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                
                use RegisterType::C;
                if let C = self.cpu_ctx.instruction.register_1 {
                        self.cpu_ctx.mem_dest |= 0xFF00;
                }
            },
            RMr => {
                let mut address: u16 = self.read(self.cpu_ctx.instruction.register_2);

                if RegisterType::C == self.cpu_ctx.instruction.register_2 {
                    address |= 0xFF00;
                }

                self.cpu_ctx.fetched_data = bus.read(address, self) as u16;
                self.emu_cycles(1, bus);
            }
            RHli => {
                self.cpu_ctx.fetched_data = bus.read(self.read(self.cpu_ctx.instruction.register_2), self) as u16;
                self.emu_cycles(1, bus);
                self.set_reg(RegisterType::HL, self.read(RegisterType::HL).wrapping_add(1))
                
            },
            RHld => {
                self.cpu_ctx.fetched_data = bus.read(self.read(self.cpu_ctx.instruction.register_2), self) as u16;
                self.emu_cycles(1, bus);
                self.set_reg(RegisterType::HL, self.read(RegisterType::HL).wrapping_sub(1));
            },
            HliR => {
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.set_reg(RegisterType::HL, self.read(RegisterType::HL).wrapping_add(1));
            },
            HldR => {
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.set_reg(RegisterType::HL, self.read(RegisterType::HL).wrapping_sub(1));
            },
            RA8 => {
                self.cpu_ctx.fetched_data = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);
                self.pc = self.pc.wrapping_add(1);
            }, 
            A8R => {
                self.cpu_ctx.mem_dest = (bus.read(self.pc, self) as u16) | 0xFF00;
                self.cpu_ctx.dest_is_mem = true;
                self.emu_cycles(1, bus);
                self.pc = self.pc.wrapping_add(1);
            },
            HlSpr => {
                self.cpu_ctx.fetched_data = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);
                self.pc = self.pc.wrapping_add(1);
            },
            D8 => {
                self.cpu_ctx.fetched_data = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);
                self.pc = self.pc.wrapping_add(1);
            },
            A16R | D16R => {   
                let lo: u16 = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);

                let hi: u16 = bus.read(self.pc.wrapping_add(1), self) as u16;
                self.emu_cycles(1, bus);

                self.cpu_ctx.mem_dest = lo | (hi << 8);
                self.cpu_ctx.dest_is_mem = true;

                self.pc = self.pc.wrapping_add(2);
                self.cpu_ctx.fetched_data = self.read(self.cpu_ctx.instruction.register_2);
            },
            Mr => {
                self.cpu_ctx.mem_dest = self.read(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.cpu_ctx.fetched_data = bus.read(self.read(self.cpu_ctx.instruction.register_1), self) as u16;
                self.emu_cycles(1, bus);
            },
            RA16 => {
                let lo: u16 = bus.read(self.pc, self) as u16;
                self.emu_cycles(1, bus);

                let hi: u16 = bus.read(self.pc.wrapping_add(1), self) as u16;
                self.emu_cycles(1, bus);

                let address: u16 = lo | (hi << 8);

                self.pc = self.pc.wrapping_add(2);
                self.cpu_ctx.fetched_data = bus.read(address, self) as u16;
                self.emu_cycles(1, bus);
            },
            _ => { 
                println!("Unknown Addressing mode hit");
            }
        }
    }
}

