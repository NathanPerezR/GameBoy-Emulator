use crate::util::*;


#[derive(Clone, Copy, Debug)]
pub struct RegisterData {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Default for RegisterData {
    fn default() -> Self {
        RegisterData {
            a: 0, 
            b: 0, 
            c: 0, 
            d: 0, 
            e: 0, 
            f: 0, 
            h: 0, 
            l: 0, 
            pc: 0x100,
            sp: 0x01,
        }
    }
}


#[derive(Clone,Copy,Debug)]
pub enum RegisterType {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}

impl Default for RegisterType {
    fn default() -> Self {
        RegisterType::A
    }
}

impl RegisterData {
    
    pub fn read_reg(&self, register_type: RegisterType) -> u16 {
        use RegisterType::*;
        match register_type {
            A => self.a.into(),
            B => self.b.into(),
            C => self.c.into(),
            D => self.d.into(),
            E => self.e.into(),
            F => self.f.into(),
            H => self.h.into(),
            L => self.l.into(),
            AF => (self.a as u16) << 8 | (self.f as u16),
            BC => (self.b as u16) << 8 | (self.c as u16),
            DE => (self.d as u16) << 8 | (self.e as u16),
            HL => (self.h as u16) << 8 | (self.l as u16), 
            PC => self.pc, 
            SP => self.sp,
        } 
    }


    pub fn write_reg(&mut self, register_type: RegisterType, value: u16) {
        use RegisterType::*;
        match register_type {
            A => self.a = value as u8,
            B => self.b = value as u8,
            C => self.c = value as u8,
            D => self.d = value as u8,
            E => self.e = value as u8,
            F => self.f = value as u8,
            H => self.h = value as u8,
            L => self.l = value as u8,
            AF => {
                self.a = (value >> 8) as u8;
                self.f = (value & 0xFF) as u8;
            },
            BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            },
            DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            },
            HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            },
            PC => self.pc = value,
            SP => self.sp = value,
        }
    }


    pub fn set_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        if z {
            self.f = bit_set(self.f, 7, true);
        }
        if n {
            self.f = bit_set(self.f, 6, true);
        }
        if h {
            self.f = bit_set(self.f, 5, true);
        }
        if c {
            self.f = bit_set(self.f, 4, true);
        }
    }

}

