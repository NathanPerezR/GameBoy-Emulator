use crate::cpu::register::Register8::{A,B,C,D,E,H,L,F};
use crate::cpu::register::Register16::{AF,BC,DE,HL,SP};
use crate::cpu::Cpu;

// addressses used throughout the program
#[derive(Clone, Copy, Debug)]
pub enum Address {
    HL,
    HLD,
    HLI,
    BC, // Value at address in BC register
    DE, // value at address in DE register
    Direct, // nn in docs, using direct since its value at nn
    ZeroPage,
    ZeroPageC,
}

// identifys when an 8 bit number is taken as input
#[derive(Clone,Copy,Debug)]
pub struct Immediate8;

impl Cpu {

    pub fn decode_exe_fetch(&mut self) {
    
        match self.opcode {
            // put value nn into n, use with B C D E H L BC DE HL SP
            0x06 => self.load_8(B, Immediate8),
            0x0E => self.load_8(C, Immediate8), 
            0x16 => self.load_8(D, Immediate8),
            0x1E => self.load_8(E, Immediate8),
            0x26 => self.load_8(H, Immediate8),
            0x2E => self.load_8(L, Immediate8),

            // Put value r2 into r1
            // use with r1, r2 = A B C D E H L (HL)
            0x7F => self.load_8(A, A),
            0x78 => self.load_8(A, B),
            0x79 => self.load_8(A, C),
            0x7A => self.load_8(A, D),
            0x7B => self.load_8(A, E),
            0x7C => self.load_8(A, L),
            0x7D => self.load_8(A, Address::HL),
            0x7E => self.load_8(A, Address::HL),
            0x40 => self.load_8(B, B),
            0x41 => self.load_8(B, C),
            0x42 => self.load_8(B, D),
            0x43 => self.load_8(B, E),
            0x44 => self.load_8(B, H),
            0x45 => self.load_8(B, L),
            0x46 => self.load_8(B, Address::HL),
            0x48 => self.load_8(C, B),
            0x49 => self.load_8(C, C),
            0x4A => self.load_8(C, D),
            0x4B => self.load_8(C, E),
            0x4C => self.load_8(C, H),
            0x4D => self.load_8(C, L),
            0x4E => self.load_8(C, Address::HL),
            0x50 => self.load_8(D, B),
            0x51 => self.load_8(D, C),
            0x52 => self.load_8(D, D),
            0x53 => self.load_8(D, E),
            0x54 => self.load_8(D, H),
            0x55 => self.load_8(D, L),
            0x56 => self.load_8(D, Address::HL),
            0x58 => self.load_8(E, B),
            0x59 => self.load_8(E, C),
            0x5A => self.load_8(E, D),
            0x5B => self.load_8(E, E),
            0x5C => self.load_8(E, H),
            0x5D => self.load_8(E, L),
            0x5E => self.load_8(E, Address::HL),
            0x60 => self.load_8(H, B),
            0x61 => self.load_8(H, C),
            0x62 => self.load_8(H, D),
            0x63 => self.load_8(H, E),
            0x64 => self.load_8(H, H),
            0x65 => self.load_8(H, L),
            0x66 => self.load_8(H, Address::HL),
            0x68 => self.load_8(L, B),
            0x69 => self.load_8(L, C),
            0x6A => self.load_8(L, D),
            0x6B => self.load_8(L, E),
            0x6C => self.load_8(L, H),
            0x6D => self.load_8(L, L),
            0x6E => self.load_8(L, Address::HL),
            0x70 => self.load_8(Address::HL,B),
            0x71 => self.load_8(Address::HL,C),
            0x72 => self.load_8(Address::HL,D),
            0x73 => self.load_8(Address::HL,E),
            0x74 => self.load_8(Address::HL,H),
            0x75 => self.load_8(Address::HL,L),
            0x36 => self.load_8(Address::HL, Immediate8), 

            // LD A, n 
            // put value n into a
            // n = A B C D E H L BC DE HL nn #
            // nn = two byte immeditate value LS byte first
            0x0A => self.load_8(A, Address::BC),
            0x1A => self.load_8(A, Address::DE),
            0x7E => self.load_8(A, Address::HL),
            0xFA => self.load_8(A, Address::Direct), // nn is second parm
            0x3E => self.load_8(A, Immediate8), // # is the second value
            
            // LD n, A
            // put value A into n
            // n = A B C D E H L BC DE HL nn
            // nn = two byte immediate value (LS byte first)
            0x47 => self.load_8(B,A),
            0x4F => self.load_8(C,A),
            0x57 => self.load_8(D,A),
            0x5F => self.load_8(E,A),
            0x67 => self.load_8(H,A),
            0x6F => self.load_8(L,A),
            0x02 => self.load_8(BC,A),
            0x12 => self.load_8(DE,A),
            0x77 => self.load_8(HL,A),
            0xEA => self.load_8(Address::Direct ,A), // first value is nn
            
            // LD A(C) 
            // Put value at address &FF00 + Register C into A
            // note: same as LD A, ($FF00 + C)
            0xF2 => self.load_8(A, Address::ZeroPageC),

            //LD (C) A
            //Put A into Address $FF00 + register C
            0xE2 => self.load_8(A, Address::ZeroPageC),

            // LD A,(HLD) same as LDD A, (HL)
            // LD A,(HL-) same as LDD A, (HL)
            // LDD A,(HL) put value at address HL into A decrement HL 
            // same as LD A, (HL) - DEC HL
            0x3A => self.load_8(A, Address::HL),

            // LD HLD, A: same as LDD HL, A
            // LD HL-, A: same as LDD HL, A
            // LDD HL, A: Put A into memory address HL, HL-- 
            0x32 => self.load_8(Address::HL, A),

            // LD A HLI: same as LDI A HL
            // LD A HL+: same as LDI A HL
            // LDI A HL: put value at HL into A, HL++ Same as LD A, HL - INC HL
            0x2A => self.load_8(A, Address::HLI),

            // LD HLI A: same as LDI HL A 
            // LD HL+ A: same as LDI HL A 
            // LDI HL A: put A into memory address HL. HL++ Same as LD HL, A - INC HL
            0x22 => self.load_8(Address::HLI, A),

            // LDH (n), A
            // Put A into memory address $FF00 + n
            // n = one byte immediate value 
            0xE0 => self.load_8(Address::ZeroPage, A),

            // LDH A,(n)
            // Put Memory address $FF00+n into A 
            // n = one byte immediate value 
            0xF0 => self.load_8(A, Address::ZeroPage), 
            


            // 16 bit loads
            // LD n, nn 
            // put value nn into n 
            // n = BC DE HL SP 
            // nn = 16-bit immediate value
            0x01 => self.load_16_immediate(BC), 
            0x11 => self.load_16_immediate(DE), 
            0x21 => self.load_16_immediate(HL), 
            0x31 => self.load_16_immediate(SP),

            // LD SP, HL 
            // Put HL into Stack pointer (SP)
            0xF9 => self.load_sp_hl(), 

            // LD HL,SP+n 
            // Same as LDHL SP, n
            
            // LDHL SP, n
            // Put SP + n effective address into HL
            // n = one byte signed immediate value
            // Flags affected: Z and N reset.  H and C set or reset according to operation 
            0xF8 => self.load_16_sp_nn(), //TODO: double check if corredt

            // LD (nn) SP
            // Put stack pointer SP at addresss n
            // nn = two byte immediate address
            0x08 => self.load_16_nn_sp(),

            // PUSH nn
            // push register pair nn onto stack
            // decreent stack pointer SP twice 
            // nn = AF BC DE HL 
            0xF5 => self.push_16(AF),
            0xC5 => self.push_16(BC),
            0xD5 => self.push_16(DE),
            0xE5 => self.push_16(Address::HL),
            
            // POP non
            // pop two bytes off stack into register pair nn Increment stack pointer twice
            // nn = AF BC DE HL
            0xF1 => self.pop_16(AF),
            0xC1 => self.pop_16(BC),
            0xD1 => self.pop_16(DE),
            0xE1 => self.pop_16(Address::HL),



            // 8-bit ALU
            // ADD A, n
            // n = A B C D E H L HL # 
            // FLAGS: Z if result 0 N reset H set if carry from bit 3 C set if carry from bit 7
            0x87 =>  self.add_8(A), 
            0x80 =>  self.add_8(B), 
            0x81 =>  self.add_8(C), 
            0x82 =>  self.add_8(D), 
            0x83 =>  self.add_8(E), 
            0x84 =>  self.add_8(H), 
            0x85 =>  self.add_8(L), 
            0x86 =>  self.add_8(Address::HL), 
            0xC6 =>  self.add_8(Immediate8), 

            // ADC A, n
            // Add n + carry flag to A
            // n = A B C D E H L (HL)
            // Z if result 0, n resset, h set if carry from 3, c set if carry from 7
            0x8F => self.adc_8(A),
            0x88 => self.adc_8(B),
            0x89 => self.adc_8(C),
            0x8A => self.adc_8(D),
            0x8B => self.adc_8(E),
            0x8C => self.adc_8(H),
            0x8D => self.adc_8(L),
            0x8E => self.adc_8(Address::HL),
            0xCE => self.adc_8(Immediate8),

            // SUB n
            // Subtract n from A
            // n = A B C D E H L (HL) #
            // z set if 0, n set, H set if no borrow bit from 4, C set if no borrow
            0x97 => self.sub_8(A),
            0x90 => self.sub_8(B),
            0x91 => self.sub_8(C),
            0x92 => self.sub_8(D),
            0x93 => self.sub_8(E),
            0x94 => self.sub_8(H),
            0x95 => self.sub_8(L),
            0x96 => self.sub_8(Address::HL),
            0xD6 => self.sub_8(Immediate8),

            _ => panic!("non covered pattern found!")
        }
    }
}
