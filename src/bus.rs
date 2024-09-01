use crate::ppu::Ppu;
use crate::cart::Cart;
use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::io::Io;

// the following memory map info is from the gbdev.io pandocs
//
// START END  DESCRIPTION                     NOTES 
// 0000	3FFF	16 KiB ROM bank 00              From cartridge, usually a fixed bank
// 4000	7FFF	16 KiB ROM Bank 01–NN           From cartridge, switchable bank via mapper (if any)
// 8000	9FFF	8 KiB Video RAM (VRAM)          In CGB mode, switchable bank 0/1
// A000	BFFF	8 KiB External RAM              From cartridge, switchable bank if any
// C000	CFFF	4 KiB Work RAM (WRAM)	
// D000	DFFF	4 KiB Work RAM (WRAM)	          In CGB mode, switchable bank 1–7
// E000	FDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
// FE00	FE9F	Object attribute memory (OAM)	
// FEA0	FEFF	Not Usable                      Nintendo says use of this area is prohibited.
// FF00	FF7F	I/O Registers	
// FF80	FFFE	High RAM (HRAM)	
// FFFF	FFFF	Interrupt Enable register (IE)

pub struct Bus {
    pub cart: Cart,
    pub ram: Ram,
    pub ppu: Ppu, 
    pub io: Io,
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            cart: Cart::default(),
            ram: Ram::new(),
            ppu: Ppu::new(),
            io: Io::default(),
        }
    }
}

impl Bus {

    pub fn read(&self, address: u16, cpu: &Cpu) -> u8 {
        
        if address < 0x8000 {
            return self.cart.cart_read(address);
        }
        else if address < 0xA000 {
            return self.ppu.vram_read(address);
        }
        else if address < 0xC000 {
            return self.cart.cart_read(address);
        }
        else if address < 0xE000 {
            return self.ram.wram_read(address);
        }
        else if address < 0xFE00 {
            //reserved echo RAM
            return 0;
        }
        else if address < 0xFEA0 {
            return self.ppu.oam_read(address);
        }
        else if address < 0xFF00 {
            // reserved 
            return 0;
        }
        else if address < 0xFF80 {
            return self.io.read(address, cpu);
        }
        else if address == 0xFFFF {
            return cpu.get_ie_register();
        }

        self.ram.hram_read(address)
        
    }

    pub fn write(&mut self, address: u16, value: u8, cpu: &mut Cpu) {

        if address < 0x8000 {
            self.cart.cart_write(address, value);
        }
        else if address < 0xA000 {
            self.ppu.vram_write(address, value);
        }
        else if address < 0xC000 {
            self.cart.cart_write(address, value);
        }
        else if address < 0xE000 {
            self.ram.wram_write(address, value);
        }
        else if address < 0xFE00 {
            //reserved echo RAM
        }
        else if address < 0xFEA0 {
            self.ppu.oam_write(address, value);
        }
        else if address < 0xFF00 {
            // reserved 
        }
        else if address < 0xFF80 {
            self.io.write(address, value, cpu);
        }
        else if address == 0xFFFF {
            cpu.set_ie_register(value);
        }
        else {
            self.ram.hram_write(address, value);
        }

        //TODO
    }

    // pub fn read16(&mut self, address: u16, cpu: &mut Cpu) -> u16 {
    //     let lo: u16 = self.read(address, cpu) as u16;
    //     let hi: u16 = self.read(address.wrapping_add(1), cpu) as u16;

    //     lo | (hi << 8)
    // }

    pub fn write16(&mut self, address: u16, value: u16, cpu: &mut Cpu) {
        self.write(address + 1, ((value >> 8) & 0xFF).try_into().unwrap(), cpu);
        self.write(address, (value & 0xFF).try_into().unwrap(), cpu);
    }
}
