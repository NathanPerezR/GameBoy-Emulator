struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
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

pub enum Register {
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
    SP,
    PC,
}

// all registars are 8 bit unsigned ints
struct RegisterData {
    a: u8,            // accumulator registar
    b: u8,            // 8 bit registar
    c: u8,            // 8 bit registar
    d: u8,            // 8 bit registar 
    e: u8,            // 8 bit registar
    f: FlagsRegister, // flag registar
    h: u8,            // 8 bit registar
    l: u8,            // 8 bit registar
    sp: u16,          // stack pointer
    pc: u16,          // program counter
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
    pub fn read(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::F => u8::from(&self.f),
            Register::H => self.h,
            Register::L => self.l,
            _ => panic!("Register not found: 8-bit")
        }
    }
    
    // reads as u16 register info
    pub fn read_u16(&self, reg: Register) -> u16 {
        match reg {
           Register::AF => (self.a as u16) << 8 | (u8::from(&self.f) as u16),
           Register::BC => (self.b as u16) << 8 | (self.c as u16),
           Register::DE => (self.d as u16) << 8 | (self.e as u16), 
           Register::HL => (self.h as u16) << 8 | (self.l as u16), 
           Register::SP => self.sp,
           Register::PC => self.pc,
           _ => panic!("Register not found for read: 16-bit")
        }
    }

    pub fn write(&mut self, reg: Register, value: u8) {
        match reg {
           Register::A => self.a = value, 
           Register::B => self.b = value, 
           Register::C => self.c = value, 
           Register::D => self.d = value, 
           Register::E => self.e = value, 
           Register::F => self.f = value.into(), 
           Register::H => self.h = value, 
           Register::L => self.l = value, 
           _ => panic!("Register can not be found for write: 8-bit")
        }
    } 

    pub fn write_u16(&mut self, reg: Register, value: u16) {

        //TODO: impliment the hi and low
        let hi = ;
        let lo = 0;

        match reg {
            Register::AF => {
                self.a = hi;
                self.f = lo.into();
            },
            Register::BC => {
                self.b = hi;
                self.c = lo;
            },
            Register::DE => {
                self.d = hi;
                self.e = lo;
            },
            Register::HL => {
                self.h = hi;
                self.e = lo;
            },
            Register::SP => self.sp = value,
            Register::PC => self.pc = value,
            _ => panic!("Register can not be found for write 16-bit")
        }
    }
}
