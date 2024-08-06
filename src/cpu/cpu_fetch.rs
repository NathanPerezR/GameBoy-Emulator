use crate::cpu::{Cpu, AddressMode, register::RegisterType};
use crate::bus::bus_read;
use crate::cart::Cart;

impl Cpu {
    // fetching data depends on the current mode of the CPU
    pub fn fetch_data(&mut self, cart: &Cart) {
        self.cpu_ctx.mem_dest = 0;
        self.cpu_ctx.dest_is_mem = false;
        
        use AddressMode::*;
        match &self.cpu_ctx.instruction.mode {
            Imp => (),                                
            R => {                                                
                self.cpu_ctx.fetched_data = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
            },
            RR => {
            }
            RD8 => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            RD16 | D16 => {
                let lo: u8 = bus_read(cart, self.registers.pc);
                self.emu_cycles(1);
                                                                                                          
                let hi: u8 = bus_read(cart, self.registers.pc + 1);
                self.emu_cycles(1);
                self.registers.pc += 2;
                                                                                                          
                self.cpu_ctx.fetched_data = ((hi as u16) << 8) | (lo as u16); 
            },
            MrR => {
                self.cpu_ctx.fetched_data = self.registers.read_reg(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                
                use RegisterType::C;
                if let C = self.cpu_ctx.instruction.register_1 {self.cpu_ctx.mem_dest |= 0xFF00}
            },
            RMr => {
                let mut address: u16 = self.registers.read_reg(self.cpu_ctx.instruction.register_2);

                if let RegisterType::C = self.cpu_ctx.instruction.register_2 {address |= 0xFF00}

                self.cpu_ctx.fetched_data = bus_read(cart, address) as u16;
                self.emu_cycles(1);
            }
            RHli => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.read_reg(self.cpu_ctx.instruction.register_2)) as u16;
                self.emu_cycles(1);
                self.registers.set_reg(RegisterType::HL, self.registers.read_reg(RegisterType::HL) + 1)
                
            },
            RHld => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.read_reg(self.cpu_ctx.instruction.register_2)) as u16;
                self.emu_cycles(1);
                self.registers.set_reg(RegisterType::HL, self.registers.read_reg(RegisterType::HL) - 1);
            },
            HliR => {
                self.cpu_ctx.fetched_data = self.registers.read_reg(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.registers.set_reg(RegisterType::HL, self.registers.read_reg(RegisterType::HL) + 1);
            },
            HldR => {
                self.cpu_ctx.fetched_data = self.registers.read_reg(self.cpu_ctx.instruction.register_2);
                self.cpu_ctx.mem_dest = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.registers.set_reg(RegisterType::HL, self.registers.read_reg(RegisterType::HL) - 1)
            },
            RA8 => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            }, 
            A8R => {
                self.cpu_ctx.mem_dest = bus_read(cart, self.registers.pc) as u16;
                self.cpu_ctx.dest_is_mem = true;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            HlSpr => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            D8 => {
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);
                self.registers.pc += 1;
            },
            A16R | D16R => {   
                let lo: u16 = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);

                let hi: u16 = bus_read(cart, self.registers.pc + 1) as u16;
                self.emu_cycles(1);

                self.cpu_ctx.mem_dest = lo | (hi << 8);
                self.cpu_ctx.dest_is_mem = true;

                self.registers.pc += 2;
                self.cpu_ctx.fetched_data = self.registers.read_reg(self.cpu_ctx.instruction.register_2);
            },
            Mr => {
                self.cpu_ctx.mem_dest = self.registers.read_reg(self.cpu_ctx.instruction.register_1);
                self.cpu_ctx.dest_is_mem = true;
                self.cpu_ctx.fetched_data = bus_read(cart, self.registers.read_reg(self.cpu_ctx.instruction.register_1)) as u16;
            },
            RA16 => {
                let lo: u16 = bus_read(cart, self.registers.pc) as u16;
                self.emu_cycles(1);

                let hi: u16 = bus_read(cart, self.registers.pc + 1) as u16;
                self.emu_cycles(1);

                let address: u16 = lo | (hi << 8);
                self.cpu_ctx.dest_is_mem = true;

                self.registers.pc += 2;
                self.cpu_ctx.fetched_data = bus_read(cart, address) as u16;
                self.emu_cycles(1);
            },
            _ => { 
                println!("Unknown Addressing mode hit");
            }
        }
    }
}

