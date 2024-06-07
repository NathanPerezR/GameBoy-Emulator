use crate::cpu::register::Register8::{A,B,C,D,E,H,L,F};
use crate::cpu::register::Register16::{AF,BC,DE,HL,SP};
use crate::cpu::Cpu;


impl Cpu {

    pub fn decode_exe_fetch(&mut self) {
    
        match self.opcode {
            // 8-BIT LOADS
            // put value nn into n, use with B C D E H L BC DE HL SP
            0x06 => self.load(B),
            0x0E => self.load(C), 
            0x16 => self.load(D),
            0x1E => self.load(E),
            0x26 => self.load(H),
            0x2E => self.load(L),

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
            0x58 => self.load(E, B),
            0x59 => self.load(E, C),
            0x5A => self.load(E, D),
            0x5B => self.load(E, E),
            0x5C => self.load(E, H),
            0x5D => self.load(E, L),
            0x5E => self.load(E, HL),
            0x60 => self.load(H, B),
            0x61 => self.load(H, C),
            0x62 => self.load(H, D),
            0x63 => self.load(H, E),
            0x64 => self.load(H, H),
            0x65 => self.load(H, L),
            0x66 => self.load(H, HL),
            0x68 => self.load(L, B),
            0x69 => self.load(L, C),
            0x6A => self.load(L, D),
            0x6B => self.load(L, E),
            0x6C => self.load(L, H),
            0x6D => self.load(L, L),
            0x6E => self.load(L, HL),
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
            0x7F => self.copy(A,A),
            0x78 => self.copy(A,B), 
            0x79 => self.copy(A,C),
            0x7A => self.copy(A,D),
            0x7B => self.copy(A,E),
            0x7C => self.copy(A,H),
            0x7D => self.copy(A,L),
            0x0A => self.copy(A,BC),
            0x1A => self.copy(A,DE),
            0x7E => self.copy(A,HL),
            0xFA => self.copy(A, ), // nn is second parm
            0x3E => self.copy(A, ), // # is the second value
            
            // LD n, A
            // put value A into n
            // n = A B C D E H L BC DE HL nn
            // nn = two byte immediate value (LS byte first)
            0x7F => self.copy(A,A),
            0x47 => self.copy(B,A),
            0x4F => self.copy(C,A),
            0x57 => self.copy(D,A),
            0x5F => self.copy(E,A),
            0x67 => self.copy(H,A),
            0x6F => self.copy(L,A),
            0x02 => self.copy(BC,A),
            0x12 => self.copy(DE,A),
            0x77 => self.copy(HL,A),
            0xEA => self.copy( ,A), // first value is nn
            
            // LD A(C) 
            // Put value at address &FF00 + Register C into A
            // note: same as LD A, ($FF00 + C)
            0xF2 => ,

            //LD (C) A
            //Put A into Address $FF00 + register C
            0xE2 => ,

            // LD A,(HLD) same as LDD A, (HL)
            // LD A,(HL-) same as LDD A, (HL)
            // LDD A,(HL) put value at address HL into A decrement HL 
            // same as LD A, (HL) - DEC HL
            0x3A => ,

            // LD HLD, A: same as LDD HL, A
            // LD HL-, A: same as LDD HL, A
            // LDD HL, A: Put A into memory address HL, HL-- 
            0x32 => ,

            // LD A HLI: same as LDI A HL
            // LD A HL+: same as LDI A HL
            // LDI A HL: put value at HL into A, HL++ Same as LD A, HL - INC HL
            0x2A => ,

            // LD HLI A: same as LDI HL A 
            // LD HL+ A: same as LDI HL A 
            // LDI HL A: put A into memory address HL. HL++ Same as LD HL, A - INC HL
            0x22 => ,

            // LDH (n), A
            // Put A into memory address $FF00 + n
            // n = one byte immediate value 
            0xE0 => ,

            // LDH A,(n)
            // Put Memory address $FF00+n into A 
            // n = one byte immediate value 
            0xF0 => ,
            


            // 16 bit loads
            // LD n, nn 
            // put value nn into n 
            // n = BC DE HL SP 
            // nn = 16-bit immediate value
            0x01 => , 
            0x11 => , 
            0x21 => , 
            0x31 => ,

            // LD SP, HL 
            // Put HL into Stack pointer (SP)
            0xF9 => , 

            // LD HL,SP+n 
            // Same as LDHL SP, n
            
            // LDHL SP, n
            // Put SP + n effective address into HL
            // n = one byte signed immediate value
            // Flags affected: Z and N reset.  H and C set or reset according to operation 
            0xF8 => ,

            // LD (nn) SP
            // Put stack pointer SP at addresss n
            // nn = two byte immediate address
            0x08 => ,

            // PUSH nn
            // push register pair nn onto stack
            // decreent stack pointer SP twice 
            // nn = AF BC DE HL 
            0xF5 => self.push(AF),
            0xC5 => self.push(BC),
            0xD5 => self.push(DE),
            0xE5 => self.push(HL),
            
            // POP non
            // pop two bytes off stack into register pair nn Increment stack pointer twice
            // nn = AF BC DE HL
            0xF1 => self.pop(AF),
            0xC1 => self.pop(BC),
            0xD1 => self.pop(DE),
            0xE1 => self.pop(HL),



            // 8-bit ALU
            // ADD A, n
            // n = A B C D E H L HL # 
            // FLAGS: Z if result 0 N reset H set if carry from bit 3 C set if carry from bit 7
            0x87 =>, 
            0x80 =>, 
            0x81 =>, 
            0x82 =>, 
            0x83 =>, 
            0x84 =>, 
            0x85 =>, 
            0x86 =>, 
            0xC6 =>, 

            
            _ => panic!("non covered pattern found!")
        }
    }

