use crate::cpu::register::RegisterType::*;
use crate::cpu::structs::{AddressMode, ConditionType::{None, Nz, Z, Nc, C}};
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
    
        match self.cpu_ctx.current_opcode {
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
            0xF8 => self.load_16_sp_nn(), 

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
            0xE5 => self.push_16(HL),
            
            // POP non
            // pop two bytes off stack into register pair nn Increment stack pointer twice
            // nn = AF BC DE HL
            0xF1 => self.pop_16(AF),
            0xC1 => self.pop_16(BC),
            0xD1 => self.pop_16(DE),
            0xE1 => self.pop_16(HL),



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

            // SUB A, n
            // Subtract n + carry flag from A
            // n = A B C D E H L (HL) #
            // set Z if result 0, set N, Set h if no borrow from bit 4, set C if no borrow
            0x9F => self.sbc_8(A),
            0x98 => self.sbc_8(B),
            0x99 => self.sbc_8(C),
            0x9A => self.sbc_8(D),
            0x9B => self.sbc_8(E),
            0x9C => self.sbc_8(H),
            0x9D => self.sbc_8(L),
            0x9E => self.sbc_8(Address::HL),

            // AND n,
            // Logically ANd n with A, result in A
            // n = A B C D E H L (HL) #
            // z if result 0, n reset, h set, c reset
            0xA7 => self.and_8(A),
            0xA0 => self.and_8(B),
            0xA1 => self.and_8(C),
            0xA2 => self.and_8(D),
            0xA3 => self.and_8(E),
            0xA4 => self.and_8(H),
            0xA5 => self.and_8(L),
            0xA6 => self.and_8(Address::HL),
            0xE6 => self.and_8(Immediate8),

            // OR n,
            // Logical Or n with register A, result in A
            // n = A B C D E H L (HL) #
            // z set if result is 0, n reset, h reset, c reset
            0xB7 => self.or_8(A), 
            0xB0 => self.or_8(B), 
            0xB1 => self.or_8(C), 
            0xB2 => self.or_8(D), 
            0xB3 => self.or_8(E), 
            0xB4 => self.or_8(H), 
            0xB5 => self.or_8(L), 
            0xB6 => self.or_8(Address::HL), 
            0xF6 => self.or_8(Immediate8),

            // XOR n,
            // Logical exclusive or n with register A, result in A
            // n = A B C D E H L (HL) #
            // Z if result is 0, reset N H and C
            // 0xAF => self.xor_8(A),
            // 0xA8 => self.xor_8(B),
            // 0xA9 => self.xor_8(C),
            // 0xAA => self.xor_8(D),
            // 0xAB => self.xor_8(E),
            // 0xAC => self.xor_8(H),
            // 0xAD => self.xor_8(L),
            0xAE => self.xor_8(Address::HL),
            0xEE => self.xor_8(Immediate8),

            // CP, n
            // compare A with n.  Basically an A-n subtraction instruction but the results are
            // thrown away
            // use with n = A B C D E H L (HL) #
            // FLAGS AFFECTED: 
            // Z: Set if A is Zero (A == n)
            // N: Set
            // H: Set if no borrow from bit 4 
            // C Set for no borrow  (A < n)
            0xBF => self.cp_8(A),
            0xB8 => self.cp_8(B),
            0xB9 => self.cp_8(C),
            0xBA => self.cp_8(D),
            0xBB => self.cp_8(E),
            0xBC => self.cp_8(H),
            0xBD => self.cp_8(L),
            0xBE => self.cp_8(Address::HL),
            0xFE => self.cp_8(Immediate8),

            // INC n 
            // Increment register n 
            // n = A B C D E H L (HL)
            // set z if result is zero, resset n, set h if carry from bit three, c not affected
            0x3C => self.inc_8(A),
            0x04 => self.inc_8(B),
            0x0C => self.inc_8(C),
            0x14 => self.inc_8(D),
            0x1C => self.inc_8(E),
            0x24 => self.inc_8(H),
            0x2C => self.inc_8(L),
            0x34 => self.inc_8(Address::HL),

            // DEC n
            // decrement register n 
            // n = A B C D E H L (HL)
            // Z set if result is 0, N set, H set if no borrow from bit 4, C not affected
            0x3D => self.dec_8(A),
            0x05 => self.dec_8(B),
            0x0D => self.dec_8(C),
            0x15 => self.dec_8(D),
            0x1D => self.dec_8(E),
            0x25 => self.dec_8(H),
            0x2D => self.dec_8(L),
            0x35 => self.dec_8(Address::HL),



            // 16-BIT ARITHMETIC 
            
            // ADD HL, n
            // add n to HL 
            // n = BC DE HL SP 
            // Z Not affected, N reset, H Set if carry from bit 11
            0x09 => self.add_hl_16(BC),
            0x19 => self.add_hl_16(DE),
            0x29 => self.add_hl_16(HL),
            0x39 => self.add_hl_16(SP),

            // ADD SP, n
            // add n to stack pointer
            // n = one byte signed immediate value (#)
            // z reset, n reset, h set or reset, c set or reset 
            0xE8 => self.add_sp_16(),

            // INC nn 
            // increment register nn 
            // use with 
            // nn = BC DE HL SP
            0x03 => self.inc_16(BC),
            0x13 => self.inc_16(DE),
            0x23 => self.inc_16(HL),
            0x33 => self.inc_16(SP),

            // DEC nn 
            // decrement register nn 
            // nn = BC DE HL SP
            0x0B => self.dec_16(BC),
            0x1B => self.dec_16(DE),
            0x2B => self.dec_16(HL),
            0x3B => self.dec_16(SP),

            // DAA 
            // Decimal adjust register A.  This instruction adjusts register A so that the correct
            // representation of Binary coded decimal (BCD) is obtained 
            // Z set if A is zero N Not affected H Reset C Set or reset according to op
            0x27 => self.daa(),

            // CPL
            // Complement A Register (flip all bits)
            // z and c not affected, set n and h
            0x2F => self.cpl(),
            
            // CCF 
            // complement carry flag 
            // if c flag is set then reset 
            // is c flag is reset then set 
            // z not affected, n reset, h reset, c Complement
            0x3F => self.ccf(),

            // SCF 
            // set carry flag
            // z not affect, n reset, h reset, c set
            0x37 => self.scf(),

            // NOP 
            // No operation 
            0x00 => self.nop(), 

            // HALT
            // Power down CPUY until an interrupt occurs
            0x76 => self.halt(),

            // STOP 
            // Halt CPU & LCD display until button press
            0x10 => self.stop(),

            // DI
            // disables interrupts but not immdiately.  Disabled after DI is executed
            0xF3 => self.di(),

            // EI 
            // enable interrupts.  This instruction enables interrupts 
            0xFB => self.ei(), 

            //ROTATES AND SHIFTS 
            
            // RLCA Rotate A left, Old bit 7 to carry flag
            // z set if result is 0 
            // n reset 
            // h reset 
            // c contains old 7 bit data 
            0x07 => self.rlca(), 
            
            // RLA 
            // Rotate A left through carry flag 
            // z set if result is 0, n reset, h reset, c conatins old 7 bit data 
            0x17 => self.rla(), 

            // RRCA 
            // rotate A right, old 0 bit to carry flag 
            // z set if result is zero, reset n and h, c conatins old bit 0 data 
            0x0F => self.rrca(), 

            // RRA 
            // rotate A right through carry flag 
            // z set if result is zero, n and h reset, c contains old 0 bit data  
            0x1F => self.rra(), 
            


            // JUMPS 
            // Jump to address nn 
            // use with nn = two bte immeditate value (LS byte first)
            0xC3 => self.jp(), 

            // JP cc, nn 
            // Jump to address n if the following condition is true 
            // cc == NZ, jump if Z flag is reset 
            // cc = Z, jump if Z flag is set 
            // cc = NC, Jump if C flag is reset, 
            // cc = C, Jump if C flag is set, 
            // use with nn == two byte immediate value (LS Byte First )
            0xC2 => self.jp_cc(Nz),
            0xCA => self.jp_cc(Z),
            0xD2 => self.jp_cc(Nc), 
            0xDA => self.jp_cc(C), 
            
            // JP (HL)
            // Jump to addres scontained in HL 
            0xE9 => self.jp_hl(),

            // JP n 
            // add n to current address and jump to it 
            // n = one byte signed immediate value 
            0x18 => self.jr(),

            // JR cc, n 
            // If the following condition is ture then add n to current address and jump to it 
            // n = one byte immeditate value 
            // cc == NZ, jump if Z flag is set 
            // cc = Z, jump if Z flag is set 
            // cc = NC, Jump if C flag is reset 
            // cc = C, Jump if C flag is set 
            0x20 => self.jr_cc(Nz),
            0x28 => self.jr_cc(Z), 
            0x30 => self.jr_cc(Nc), 
            0x38 => self.jr_cc(C),

            // CALLS 
            

            // Call nn 
            // push address of next instruction onto stack and then jump to address nn 
            // nn = two byte immidate value 
            0xCD => self.call(), 

            // CALL cc, nn 
            // call address n if following condition is true 
            // cc == NZ, call if Z flag is reset 
            // cc == Z, call if Z flag is set 
            // cc = NC, call if C FLAG is reset 
            // cc = C call if C flag is set 
            // nn == two byte immeditate value (LS byte first )
            0xC4 => self.call_cc(Nz),
            0xCC => self.call_cc(Z),
            0xD4 => self.call_cc(Nc),
            0xDC => self.call_cc(C),

            

            // RESTARTS 

            // RST n 
            // Push present address onto the stack 
            // jump to address $0000 + n 
            // use with n = $00 $08 $18 $20 $20 $28 $30 $38 
            0xC7 => self.rst(0x00),
            0xCF => self.rst(0x08),
            0xD7 => self.rst(0x10),
            0xDF => self.rst(0x18),
            0xE7 => self.rst(0x20),
            0xEF => self.rst(0x28),
            0xF7 => self.rst(0x30),
            0xFF => self.rst(0x38),

            // RETURNS 
            
            // RET 
            // pop two bytes from stack & jump to that adresss 
            0xC9 => self.ret(),

            // RET cc 
            // Return if the following condition is true: 
            // cc = NZ, return if z flag is reset 
            // cc = z return if z flag is set 
            // nc return if c flag is reset 
            // cc = c return if c flag is set 
            0xC0 => self.ret_cc(Nz),
            0xC8 => self.ret_cc(Z),
            0xD0 => self.ret_cc(Nc),
            0xD8 => self.ret_cc(C),

            // RETI 
            // Pop two bytes from stack & jump to that address then enable interrupts
            0xD9 => self.reti(),

            

            _ => panic!("op code not found")
        }
    }

    pub fn cb_decode_exe_fetch(&mut self) {

        match self.cpu_ctx.current_opcode {
            // MISCELLANEOUS
           
            // SWAP n
            // Swap uper and lower nibles of n
            // n = A B C D E H L (HL)
            // Z set if zero, n h c all reset
            0x37 => self.swap_8(A),
            0x30 => self.swap_8(B),
            0x31 => self.swap_8(C),
            0x32 => self.swap_8(D),
            0x33 => self.swap_8(E),
            0x34 => self.swap_8(H),
            0x35 => self.swap_8(L),
            0x36 => self.swap_8(Address::HL),

            // RLC n 
            // use with n = A B C D E H L (HL)
            // z set if result is zero, n reset, h reset, c contains old bit 7 data 
            0x07 => self.rlc_8(A),
            0x00 => self.rlc_8(B),
            0x01 => self.rlc_8(C),
            0x02 => self.rlc_8(D),
            0x03 => self.rlc_8(E),
            0x04 => self.rlc_8(H),
            0x05 => self.rlc_8(L),
            0x06 => self.rlc_8(Address::HL),

            // RL n 
            // rotate n left through carry flag 
            // use wtih n = A B C D E H L (HL)
            // z set if result 0, n reset, h reset, c contains old 7 bit data 
            0x17 => self.rl_8(A),
            0x10 => self.rl_8(B),
            0x11 => self.rl_8(C),
            0x12 => self.rl_8(D),
            0x13 => self.rl_8(E),
            0x14 => self.rl_8(H),
            0x15 => self.rl_8(L),
            0x16 => self.rl_8(Address::HL),

            // RRC n 
            // Rotate n right. Old bit 0 to Carry flag 
            // n = A B C D E H L (HL)
            // z set if result is zero, n reset, h reset, c contains old bit 0 data 
            0x0F => self.rrc_8(A), 
            0x08 => self.rrc_8(B), 
            0x09 => self.rrc_8(C), 
            0x0A => self.rrc_8(D), 
            0x0B => self.rrc_8(E), 
            0x0C => self.rrc_8(H), 
            0x0D => self.rrc_8(L), 
            0x0E => self.rrc_8(Address::HL), 

            // RR n, 
            // Rotate n right through Carry flag 
            // z set if zero, n reset, h reset, c contains old 0 bit data 
            0x1F => self.rr_8(A),
            0x18 => self.rr_8(B),
            0x19 => self.rr_8(C),
            0x1A => self.rr_8(D),
            0x1B => self.rr_8(E),
            0x1C => self.rr_8(H),
            0x1D => self.rr_8(L),

            // sla n 
            // shift n left into carry.  LSB of n set to 0 
            // n = A B C D E H L (HL)
            // z set if result is zero, n reset, h reset, contains old bit 7 data 
            0x27 => self.sla_8(A),
            0x20 => self.sla_8(B),
            0x21 => self.sla_8(C),
            0x22 => self.sla_8(D),
            0x23 => self.sla_8(E),
            0x24 => self.sla_8(H),
            0x25 => self.sla_8(L),
            0x26 => self.sla_8(Address::HL),

            // SRA n, 
            // shift n right inot carry.  MSB doesn't change 
            // n = A B C D E H L (HL)
            // z set if result is zero, n reset, h reset, c contains old bit 0 data 
            0x2F => self.sra_8(A),
            0x28 => self.sra_8(B),
            0x29 => self.sra_8(C),
            0x2A => self.sra_8(D),
            0x2B => self.sra_8(E),
            0x2C => self.sra_8(H),
            0x2D => self.sra_8(L),
            0x2E => self.sra_8(Address::HL),

            // SRL n 
            // shift n right into carry.  MSB Set to 0 
            // use with n = A B C D E H L (HL)
            // z set if result is zero, n reset, h reset, c contains old bit 0 data 
            0x3F => self.srl_8(A),
            0x38 => self.srl_8(B),
            0x39 => self.srl_8(C),
            0x3A => self.srl_8(D),
            0x3B => self.srl_8(E),
            0x3C => self.srl_8(H),
            0x3D => self.srl_8(L),
            0x3E => self.srl_8(Address::HL),
            
            
            // BIT OPCODES 
            
            // test bit b in register r 
            // use with b = 0 - 7, r = A B C D E H L (HL)
            // z set if bit b of register r is 0, reset n, set h, c not affected 
            0x47 => self.bit(A),
            0x40 => self.bit(B),
            0x41 => self.bit(C),
            0x42 => self.bit(D),
            0x43 => self.bit(E),
            0x44 => self.bit(H),
            0x45 => self.bit(L),
            0x46 => self.bit(Address::HL),
            
            // SET b r 
            // set bit b in register r 
            // b = 0 - 7, r = A B C D E H L (HL)
            // no flags affected 
            0xC7 => self.set(A),
            0xC0 => self.set(B),
            0xC1 => self.set(C),
            0xC2 => self.set(D),
            0xC3 => self.set(E),
            0xC4 => self.set(H),
            0xC5 => self.set(L),
            0xC6 => self.set(Address::HL),

            // RES b,r 
            // b = 0 - 7, r = A B C D E H L (HL) 
            // no flags affected 
            0x87 => self.res(A), 
            0x80 => self.res(B), 
            0x81 => self.res(C), 
            0x82 => self.res(D), 
            0x83 => self.res(E), 
            0x84 => self.res(H), 
            0x85 => self.res(L), 
            0x86 => self.res(Address::HL), 


            _ => panic!("cb opcode not found ")
            
        }
    }
}
