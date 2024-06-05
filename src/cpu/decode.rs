use crate::cpu::register::Register8::{A,B,C,D,E,H,L,F};
use crate::cpu::register::Register16::{AF,BC,DE,HL,SP};
use crate::cpu::Cpu;


impl Cpu {

    pub fn decode_exe_fetch(&mut self) {
    
        match self.opcode {
            // 8-BIT LOADS
            // put value nn into n, use with B C D E H L BC DE HL SP
            0x06 => self.load(),
            0x0E => self.load(), 
            0x16 => self.load(),
            0x1E => self.load(),
            0x26 => self.load(),
            0x2E => self.load(),
            // Put value r2 into r1
            // use with r1, r2 = A B C D E H L (HL)
            0x7F => self.load(A, A),
            0x78 => self.load(A, B),
            0x79 => self.load(A, C),
            0x7A => self.load(A, D),
            0x7B => self.load(A, E),
            0x7C => self.load(A, L),
            0x7D => self.load(A, HL),
            0x7E => self.load(),
            0x40 => self.load(),
            0x41 => self.load(),
            0x42 => self.load(),
            0x43 => self.load(),
            0x44 => self.load(),
            0x45 => self.load(),
            0x46 => self.load(),
            0x4A => self.load(),
            0x4B => self.load(),
            0x4C => self.load(),
            0x4D => self.load(),
            0x4E => self.load(),
            0x50 => self.load(),
            0x51 => self.load(),
            0x52 => self.load(),
            0x53 => self.load(),
            0x54 => self.load(),
            0x55 => self.load(),
            0x56 => self.load(),
            0x58 => self.load(),
            0x59 => self.load(),
            0x5A => self.load(),
            0x5B => self.load(),
            0x5C => self.load(),
            0x5D => self.load(),
            0x5E => self.load(),
            0x60 => self.load(),
            0x61 => self.load(),
            0x62 => self.load(),
            0x63 => self.load(),
            0x64 => self.load(),
            0x65 => self.load(),
            0x66 => self.load(),
            0x68 => self.load(),
            0x69 => self.load(),
            0x6A => self.load(),
            0x6B => self.load(),
            0x6C => self.load(),
            0x6D => self.load(),
            0x6E => self.load(),
            0x70 => self.load(),
            0x71 => self.load(),
            0x72 => self.load(),
            0x73 => self.load(),
            0x74 => self.load(),
            0x75 => self.load(),
            0x36 => self.load(),

            
            _ => panic!("non covered pattern found!")
        }
    }
}
