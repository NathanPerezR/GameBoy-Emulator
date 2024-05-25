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
impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
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

// all registars are 8 bit unsigned ints
struct Registers {
    a: u8,            // accumulator registar
    b: u8,            // 8 bit registar
    c: u8,            // 8 bit registar
    d: u8,            // 8 bit registar 
    e: u8,            // 8 bit registar
    f: FlagsRegister, // flag registar
    h: u8,            // 8 bit registar
    l: u8,            // 8 bit registar
}

// 16 bit instructions use af, bc, de, and hl combonied to store 16 bits
impl Registers {

    // combine b and c, where b is the upper bits and c are the lower bits by left shifting the
    // upper bits and then OR with the lower bits
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    // set the upper 8 bits of value to b and the lower 8 bits of value toc
    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    } 

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn get_de(&self) -> u16 {                    
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    fn get_hl(&self) -> u16 {                    
        (self.h as u16) << 8 | self.l as u16
    }
}

// INSTRUCTIONS 

// registars that can be targeted by Arithmetic Instructions
enum ArithmeticTarget {
    A, B, C, D, E, H, L
}

// place where all instructions will be defined 
enum Instruction {
    ADD(ArithmeticTarget)
} 

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    Arithmet
                }
            }
        }
    }
}
