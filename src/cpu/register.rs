// enum for the 8 bit registers
#[derive(Clone,Copy,Debug)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

// enum for the 16 bit registers (upper 8 bits first reg, lower 8 second letter)
#[derive(Clone,Copy,Debug)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

// all registars are 8 bit unsigned ints
#[derive(Debug)]
pub struct RegisterData {
    pub a: u8,            // accumulator registar
    pub b: u8,            // 8 bit registar
    pub c: u8,            // 8 bit registar
    pub d: u8,            // 8 bit registar 
    pub e: u8,            // 8 bit registar
    pub f: FlagsRegister, // flag registar
    pub h: u8,            // 8 bit registar
    pub l: u8,            // 8 bit registar
    pub sp: u16,          // stack pointer
    pub pc: u16,          // program counter
}

// 16 bit instructions use af, bc, de, and hl combonied to store 16 bits
impl RegisterData {   
    pub fn new() -> RegisterData {
        RegisterData {
            a: 0, 
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {zero: false, subtract: false, half_carry: false, carry: false},
            h: 0,
            l: 0,
            sp: 0xFFFE,
            pc: 0x000,
        }
    }
    
    // reads as u8 register info
    pub fn read_8(&self, reg: Register8) -> u8 {
        use self::Register8::*;
        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            F => u8::from(&self.f),
            H => self.h,
            L => self.l,
        }
    }

    // reads as u16 register info
    pub fn read_u16(&self, reg: Register16) -> u16 {
        use self::Register16::*;
        match reg { 
           AF => (self.a as u16) << 8 | (u8::from(&self.f) as u16),
           BC => (self.b as u16) << 8 | (self.c as u16),
           DE => (self.d as u16) << 8 | (self.e as u16), 
           HL => (self.h as u16) << 8 | (self.l as u16), 
           SP => self.sp,
           PC => self.pc,
        }
    }

    // write a 8 bit value to a register
    pub fn write_u8(&mut self, reg: Register8, value: u8) {
        use self::Register8::*;
        match reg {
           A => self.a = value, 
           B => self.b = value, 
           C => self.c = value, 
           D => self.d = value, 
           E => self.e = value, 
           F => self.f = value.into(), 
           H => self.h = value, 
           L => self.l = value, 
        }
    } 
    
    // write a 16 bit value to 2 8-bit registers
    pub fn write_u16(&mut self, reg: Register16, value: u16) {

        let hi: u8 = (value >> 8) as u8; 
        let lo: u8 = (0x00FF & value) as u8;
        
        use self::Register16::*;
        match reg {
            AF => {
                self.a = hi;
                self.f = lo.into();
            },
            BC => {
                self.b = hi;
                self.c = lo;
            },
            DE => {
                self.d = hi;
                self.e = lo;
            },
            HL => {
                self.h = hi;
                self.e = lo;
            },
            SP => self.sp = value,
            PC => self.pc = value,
        }
    }

    pub fn set_zero_flag(&mut self, value: bool) {
        self.f.zero = value;
    }
    pub fn set_subtract_flag(&mut self, value: bool) {
        self.f.subtract = value;
    }
    pub fn set_half_carry_flag(&mut self, value: bool) {
        self.f.half_carry = value;
    }
    pub fn set_carry_flag(&mut self, value: bool) {
        self.f.carry = value;
    }

}

#[derive(Debug)]
struct FlagsRegister {
    zero: bool,         // Set when the result of a math operation is zero or tuo values match wehn
                        // using the CP instruction 

    subtract: bool,     // Set if subtraction was preformed in the last math instruction 

    half_carry: bool,   // This bit is set if a carry occurred from the lower nibble in the last
                        // math opperation
                        
    carry: bool,    
}

// const for the bits locations in the byte, lower 4 are always 0
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSTION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// from flag to 8 bit unsigned
impl std::convert::From<&FlagsRegister> for u8 {
    fn from(flag: &FlagsRegister) -> u8 {
        (if flag.zero        {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract    {1} else {0}) << SUBTRACT_FLAG_BYTE_POSTION |
        (if flag.half_carry  {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry       {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

// from 8 bit unsigned to flag
impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSTION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}
