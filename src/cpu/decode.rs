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
            0x7E => self.load(A, HL),
            0x40 => self.load(B, B),
            0x41 => self.load(B, C),
            0x42 => self.load(B, D),
            0x43 => self.load(B, E),
            0x44 => self.load(B, H),
            0x45 => self.load(B, L),
            0x46 => self.load(B, HL),
            0x48 => self.load(C, B),
            0x49 => self.load(C, C),
            0x4A => self.load(C, D),
            0x4B => self.load(C, E),
            0x4C => self.load(C, H),
            0x4D => self.load(C, L),
            0x4E => self.load(C, HL),
            0x50 => self.load(D, B),
            0x51 => self.load(D, C),
            0x52 => self.load(D, D),
            0x53 => self.load(D, E),
            0x54 => self.load(D, H),
            0x55 => self.load(D, L),
            0x56 => self.load(D, HL),
            0x58 => self.load(E,B),
            0x59 => self.load(E,C),
            0x5A => self.load(E,D),
            0x5B => self.load(E,E),
            0x5C => self.load(E,H),
            0x5D => self.load(E,L),
            0x5E => self.load(E,HL),
            0x60 => self.load(H,B),
            0x61 => self.load(H,C),
            0x62 => self.load(H,D),
            0x63 => self.load(H,E),
            0x64 => self.load(H,H),
            0x65 => self.load(H,L),
            0x66 => self.load(H,HL),
            0x68 => self.load(L,B),
            0x69 => self.load(L,C),
            0x6A => self.load(L,D),
            0x6B => self.load(L,E),
            0x6C => self.load(L,H),
            0x6D => self.load(L,L),
            0x6E => self.load(L,HL),
            0x70 => self.load(HL,B),
            0x71 => self.load(HL,C),
            0x72 => self.load(HL,D),
            0x73 => self.load(HL,E),
            0x74 => self.load(HL,H),
            0x75 => self.load(HL,L),
            0x36 => self.load(HL,), // TODO: takes as input n

            // LD A, n 
            // put value n into a
            // n = A B C D E H L BC DE HL nn #
            // nn = two byte immeditate value LS byte first
            
            _ => panic!("non covered pattern found!")
        }
    }
}
