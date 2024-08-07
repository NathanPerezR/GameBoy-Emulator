use crate::cart::Cart;

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

pub fn bus_read(cart: &Cart, address: u16) -> u8 {
    if address < 0x8000 {
        return cart.cart_read(address);
    }

    //TODO
    // remove false
    1
}

pub fn bus_write(cart: &mut Cart, address: u16, value: u8) {
    if address < 0x8000 {
        // ROM DATA 
        cart.cart_write(address, value);
    }

    //TODO
}

pub fn bus_read16(cart: &Cart, address: u16) -> u16 {
    let lo: u16 = bus_read(cart, address) as u16;
    let hi: u16 = bus_read(cart, address) as u16;

    lo | (hi << 8)
}

pub fn bus_write16(cart: &mut Cart, address: u16, value: u16) {
    
    let lo = (value & 0xFF) as u8;
    let hi = (value >> 8) as u8;

    bus_write(cart, address + 1, lo);
    bus_write(cart, address, hi);
}
