use crate::ppu::Ppu;
use crate::cart::Cart;
use crate::ram::Ram;

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

}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            cart: Cart::default(),
            ram: Ram::new(),
            ppu: Ppu::default(),
        }
    }
}

impl Bus {

    pub fn read(&mut self, address: u16) -> u8 {
        if address < 0x8000 {
            return self.cart.cart_read(address);
        }
        else if address < 0xA000 {
            // char / map data
        }
        else if address < 0xC00 {
            // Cart Ram
            return self.cart.cart_read(address);
        }
        else if address < 0xE000 {
            // WRAM (Working Ram) 
            return self.ram.wram_read(address)
        }
        else if address < 0xFE00 {
            //reserved echo RAM
            return 0;
        }
        else if address < 0xFEA0 {
            // OAM 
        }
        else if address < 0xFF00 {
            // reserved 
        }
        else if address < 0xFF80 {
            // IO Registers 
        }
        else if address == 0xFFFF {
    //        cpu_get_ie_register();
        }
        else {
    //        return hram_read(address);
        }

        0
    }

    pub fn write(&mut self, address: u16, value: u8) {

        if address < 0x8000 {
            // cart.cart_write(address, value);
        }
        else if address < 0xA000 {
            // char / map data
        }
        else if address < 0xC00 {
            // Cart Ram
            self.cart.cart_write(address, value);
        }
        else if address < 0xE000 {
            // WRAM (Working Ram) 
            self.ram.wram_write(address, value)
        }
        else if address < 0xFE00 {
            //reserved echo RAM
        }
        else if address < 0xFEA0 {
            // OAM 
        }
        else if address < 0xFF00 {
            // reserved 
        }
        else if address < 0xFF80 {
            // IO Registers 
        }
        else if address == 0xFFFF {
            // cpu_get_ie_register();
        }
        else {
            return self.ram.hram_read(address);
        }

        //TODO
    }

    pub fn read16(&mut self, address: u16) -> u16 {
        let lo: u16 = self.read(address) as u16;
        let hi: u16 = self.read(address) as u16;

        lo | (hi << 8)
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        
        let lo = (value & 0xFF) as u8;
        let hi = (value >> 8) as u8;

        self.write(address + 1, lo);
        self.write(address, hi);
    }
}



























































































