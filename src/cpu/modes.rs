use crate::cpu::Cpu;
use crate::cart::Cart;
use crate::cpu::RegisterType;
use crate::cpu::structs::{InstructionName, ConditionType, AddressMode, Instruction};

impl Cpu {

    pub fn instruction_by_opcode(&mut self, cart: &mut Cart) {
        
        // TODO: REMOVE THIS 
        self.cpu_ctx.fetched_data = 0x0000;
        self.cpu_ctx.instruction = Instruction::default();

        match self.cpu_ctx.current_opcode {

            // 8 bit load with immediate value
            0x06 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            },  
            0x0E => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x16 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x1E => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);


            }, 
            0x26 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x2E => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.mode = AddressMode::RD8;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
    
            // 8 bit load register & memory address
            0x7F => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x78 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x79 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x7A => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            },
            0x7B => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }
            0x7C => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x7D => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x7E => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode = AddressMode::RMr;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x40 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x41 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x42 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x43 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x44 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x45 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x46 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode = AddressMode::RMr;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x48 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x49 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x4A => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x4B => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x4C => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load);
            }, 
            0x4D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x4E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x50 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x51 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x52 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x53 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x54 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x55 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x56 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x58 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x59 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x5A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x5B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x5C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x5D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x5E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x60 => {}, 
            0x61 => {}, 
            0x62 => {}, 
            0x63 => {}, 
            0x64 => {}, 
            0x65 => {}, 
            0x66 => {}, 
            0x68 => {}, 
            0x69 => {}, 
            0x6A => {}, 
            0x6B => {}, 
            0x6C => {}, 
            0x6D => {}, 
            0x6E => {}, 
            0x70 => {}, 
            0x71 => {}, 
            0x72 => {}, 
            0x73 => {}, 
            0x74 => {}, 
            0x75 => {}, 
            0x36 => {}, 
            0x0A => {}, 
            0x1A => {}, 
            0xFA => {}, 
            0x3E => {},

            // 8 bit loads
            0x47 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x4F => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x57 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x5F => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x67 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x6F => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x02 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::MrR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x12 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::MrR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x77 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::MrR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0xEA => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::A16R;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 

            0xF2 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RMr;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0xE2 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::MrR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x3A => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode = AddressMode::RHld;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x32 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::HldR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x2A => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode = AddressMode::RHli;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x22 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::HliR;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0xE0 => {}, 
            0xF0 => {}, 
            
            // 16 bit loads
            0x01 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC; 
                self.cpu_ctx.instruction.mode = AddressMode::RD16;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x11 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode = AddressMode::RD16;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x21 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode = AddressMode::RD16;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            }, 
            0x31 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.mode = AddressMode::RD16;
                self.cpu_ctx.instruction.function = Some(Cpu::load)
            },

            0xF9 => {}, 
            0xF8 => {}, 
            0x08 => {}, 
            0xF5 => {}, 
            0xC5 => {}, 
            0xD5 => {}, 
            0xE5 => {}, 
            0xF1 => {}, 
            0xC1 => {}, 
            0xD1 => {}, 
            0xE1 => {}, 
            0x87 => {}, 
            0x80 => {}, 
            0x81 => {}, 
            0x82 => {}, 
            0x83 => {}, 
            0x84 => {}, 
            0x85 => {}, 
            0x86 => {}, 
            0xC6 => {}, 
            0x8F => {}, 
            0x88 => {}, 
            0x89 => {}, 
            0x8A => {}, 
            0x8B => {}, 
            0x8C => {}, 
            0x8D => {}, 
            0x8E => {}, 
            0xCE => {}, 
            0x97 => {}, 
            0x90 => {}, 
            0x91 => {}, 
            0x92 => {}, 
            0x93 => {}, 
            0x94 => {}, 
            0x95 => {}, 
            0x96 => {}, 
            0xD6 => {}, 
            0x9F => {}, 
            0x98 => {}, 
            0x99 => {}, 
            0x9A => {}, 
            0x9B => {}, 
            0x9C => {}, 
            0x9D => {}, 
            0x9E => {}, 
            0xA7 => {}, 
            0xA0 => {}, 
            0xA1 => {}, 
            0xA2 => {}, 
            0xA3 => {}, 
            0xA4 => {}, 
            0xA5 => {}, 
            0xA6 => {}, 
            0xE6 => {}, 
            0xB7 => {}, 
            0xB0 => {}, 
            0xB1 => {}, 
            0xB2 => {}, 
            0xB3 => {}, 
            0xB4 => {}, 
            0xB5 => {}, 
            0xB6 => {}, 
            0xF6 => {}, 

            // XOR n,
            // Logical exclusive or n with register A, result in A
            // n = A B C D E H L (HL) #
            // Z if result is 0, reset N H and C
            0xAF => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xA8 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xA9 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAA => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAB => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAC => {
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAD => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.mode = AddressMode::R;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAE => {}, 
            0xEE => {}, 

            0xBF => {}, 
            0xB8 => {}, 
            0xB9 => {}, 
            0xBA => {}, 
            0xBB => {}, 
            0xBC => {}, 
            0xBD => {}, 
            0xBE => {}, 
            0xFE => {}, 
            0x3C => {}, 
            0x04 => {}, 
            0x0C => {}, 
            0x14 => {}, 
            0x1C => {}, 
            0x24 => {}, 
            0x2C => {}, 
            0x34 => {}, 
            0x3D => {}, 
            0x05 => {}, 
            0x0D => {}, 
            0x15 => {}, 
            0x1D => {}, 
            0x25 => {}, 
            0x2D => {}, 
            0x35 => {}, 
            0x09 => {}, 
            0x19 => {}, 
            0x29 => {}, 
            0x39 => {}, 
            0xE8 => {}, 
            0x03 => {}, 
            0x13 => {}, 
            0x23 => {}, 
            0x33 => {}, 
            0x0B => {}, 
            0x1B => {}, 
            0x2B => {}, 
            0x3B => {}, 
            0x27 => {}, 
            0x2F => {}, 
            0x3F => {}, 
            0x37 => {}, 

            0x00 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Nop;
                self.cpu_ctx.instruction.mode = AddressMode::Imp;
                self.cpu_ctx.instruction.function = Some(Cpu::nop)
            }, 

            0x76 => {}, 
            0x10 => {}, 

            0xF3 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Di;
                self.cpu_ctx.instruction.mode = AddressMode::Imp;
                self.cpu_ctx.instruction.function = Some(Cpu::di);
            },

            0xFB => {}, 
            0x07 => {}, 
            0x17 => {}, 
            0x0F => {}, 
            0x1F => {},

            0xC3 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Jp;
                self.cpu_ctx.instruction.mode = AddressMode::D16;
                self.cpu_ctx.instruction.function = Some(Cpu::jp);
            },

            0xC2 => {}, 
            0xCA => {}, 
            0xD2 => {}, 
            0xDA => {}, 
            0xE9 => {}, 
            0x18 => {}, 
            0x20 => {}, 
            0x28 => {}, 
            0x30 => {}, 
            0x38 => {}, 
            0xCD => {}, 
            0xC4 => {}, 
            0xCC => {}, 
            0xD4 => {}, 
            0xDC => {}, 
            0xC7 => {}, 
            0xCF => {}, 
            0xD7 => {}, 
            0xDF => {}, 
            0xE7 => {}, 
            0xEF => {}, 
            0xF7 => {}, 
            0xFF => {}, 
            0xC9 => {}, 
            0xC0 => {}, 
            0xC8 => {}, 
            0xD0 => {}, 
            0xD8 => {}, 
            0xD9 => {}, 
            _ => {}
        }
    }
}
// CB OPCODES
// 0x37
// 0x30
// 0x31
// 0x32
// 0x33
// 0x34
// 0x35
// 0x36
// 0x07
// 0x00
// 0x01
// 0x02
// 0x03
// 0x04
// 0x05
// 0x06
// 0x17
// 0x10
// 0x11
// 0x12
// 0x13
// 0x14
// 0x15
// 0x16
// 0x0F
// 0x08
// 0x09
// 0x0A
// 0x0B
// 0x0C
// 0x0D
// 0x0E
// 0x1F
// 0x18
// 0x19
// 0x1A
// 0x1B
// 0x1C
// 0x1D
// 0x27
// 0x20
// 0x21
// 0x22
// 0x23
// 0x24
// 0x25
// 0x26
// 0x2F
// 0x28
// 0x29
// 0x2A
// 0x2B
// 0x2C
// 0x2D
// 0x2E
// 0x3F
// 0x38
// 0x39
// 0x3A
// 0x3B
// 0x3C
// 0x3D
// 0x3E
// 0x47
// 0x40
// 0x41
// 0x42
// 0x43
// 0x44
// 0x45
// 0x46
// 0xC7
// 0xC0
// 0xC1
// 0xC2
// 0xC3
// 0xC4
// 0xC5
// 0xC6
// 0x87
// 0x80
// 0x81
// 0x82
// 0x83
// 0x84
// 0x85
// 0x86
