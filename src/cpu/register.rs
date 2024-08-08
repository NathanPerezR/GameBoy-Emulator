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
            a: 0x01, 
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


#[derive(Clone,Copy,Debug, Default)]
pub enum RegisterType {
    #[default]
    None,
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

impl RegisterData {
    
    pub fn read(&self, register_type: RegisterType) -> u16 {
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
            None => {0}
        } 
    }


    pub fn write(&mut self, register_type: RegisterType, value: u16) {
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
            None => {}
        }
    }

    pub fn reverse(n: u16) -> u16 {
        ((n & 0xFF00) >> 8) | ((n & 0x00FF) << 8)
    }

    pub fn set_reg(&mut self, rt: RegisterType, val: u16) {
        match rt {
            RegisterType::A => self.a = (val & 0xFF) as u8,
            RegisterType::F => self.f = (val & 0xFF) as u8,
            RegisterType::B => self.b = (val & 0xFF) as u8,
            RegisterType::C => self.c = (val & 0xFF) as u8,
            RegisterType::D => self.d = (val & 0xFF) as u8,
            RegisterType::E => self.e = (val & 0xFF) as u8,
            RegisterType::H => self.h = (val & 0xFF) as u8,
            RegisterType::L => self.l = (val & 0xFF) as u8,

            RegisterType::AF => {
                self.a = (val >> 8) as u8; 
                self.f = (val & 0xFF) as u8; 
            }
            RegisterType::BC => {
                self.b = (val >> 8) as u8; 
                self.c = (val & 0xFF) as u8; 
            }
            RegisterType::DE => {
                self.d = (val >> 8) as u8; 
                self.e = (val & 0xFF) as u8;
            }
            RegisterType::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0xFF) as u8;
            }

            RegisterType::PC => self.pc = val,
            RegisterType::SP => self.sp = val,
            RegisterType::None => {}
        }
    }

    pub fn set_z(&mut self, z: bool ) {
        self.f = bit_set(self.f, 7, z);
    }
    pub fn set_n(&mut self, n: bool ) {
        self.f = bit_set(self.f, 6, n);         
    }
    pub fn set_h(&mut self, h: bool ) {
        self.f = bit_set(self.f, 5, h);         
    }
    pub fn set_c(&mut self, c: bool ) {
        self.f = bit_set(self.f, 4, c); 
    }                   

    pub fn get_c(&mut self) -> bool {
        nth_bit(self.f.into(), 4)
    }
}


pub fn is_16_bit(register_type: RegisterType) -> bool {
    use RegisterType::*;
    match register_type {
        A | B | C | D | E | F | L | H | None => {false},
        AF | BC | DE | HL | PC | SP => {true},
    }
}

