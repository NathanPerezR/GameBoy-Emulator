use crate::cpu::Cpu;
use crate::cpu::RegisterType;
use crate::cpu::structs::{InstructionName, ConditionType, AddressMode, Instruction};

impl Cpu {

    pub fn instruction_by_opcode(&mut self) {
        
        self.cpu_ctx.fetched_data = 0x0000;
        self.cpu_ctx.instruction = Instruction::default();

        match self.cpu_ctx.current_opcode {
            0xCB => {
                self.cpu_ctx.instruction.in_type    = InstructionName::CB;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.function   = Some(Cpu::cb);
            }
            0x06 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            },  
            0x0E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x16 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x1E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);


            }, 
            0x26 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x2E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
    
            0x7F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x78 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x79 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x7A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            },
            0x7B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }
            0x7C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x7D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x7E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x40 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x41 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x42 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x43 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x44 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x45 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x46 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x48 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x49 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x4A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x4B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x4C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
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
            0x60 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x61 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x62 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x63 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x64 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x65 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x66 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x68 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x69 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x6A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x6B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x6C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x6D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x6E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x70 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x71 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x72 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x73 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x74 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x75 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x36 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::MrD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x0A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x1A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0xFA => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RA16;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            }, 
            0x3E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::load);
            },

            0x47 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x4F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x57 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
              self.cpu_ctx.instruction.function     = Some(Cpu::load)
            }, 
            0x5F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x67 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x6F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x02 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x12 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x77 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0xEA => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::A16R;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 

            0xF2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0xE2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::MrR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x3A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RHld;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x32 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::HldR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x2A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RHli;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x22 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::HliR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 

            0xE0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ldh;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::A8R;
                self.cpu_ctx.instruction.function   = Some(Cpu::ldh)
            }, 
            0xF0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ldh;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RA8;
                self.cpu_ctx.instruction.function   = Some(Cpu::ldh)
            }, 
            
            // 16 bit loads
            0x01 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC; 
                self.cpu_ctx.instruction.mode       = AddressMode::RD16;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x11 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::RD16;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x21 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RD16;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 
            0x31 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::RD16;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            },

            0xF9 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 

            0xF8 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::HlSpr;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 

            0x08 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ld;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::A16R;
                self.cpu_ctx.instruction.function   = Some(Cpu::load)
            }, 

            0xF5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Push;
                self.cpu_ctx.instruction.register_1 = RegisterType::AF;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::push)
            }, 
            0xC5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Push;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::push)
            }, 
            0xD5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Push;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::push)
            }, 
            0xE5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Push;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::push)
            }, 
            0xF1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Pop;
                self.cpu_ctx.instruction.register_1 = RegisterType::AF;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::pop)
            }, 
            0xC1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Pop;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::pop)
            }, 
            0xD1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Pop;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::pop)
            }, 
            0xE1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Pop;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::pop)
            }, 
            0x87 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x80 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x81 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x82 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x83 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x84 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x85 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x86 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0xC6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x8F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x88 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x89 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x8A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x8B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x8C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x8D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x8E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0xCE => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Adc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::adc)
            }, 
            0x97 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x90 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x91 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x92 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x93 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x94 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x95 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x96 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0xD6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sub;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::sub)
            }, 
            0x9F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x98 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x99 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x9A => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x9B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x9C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x9D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0x9E => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Sbc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::sbc)
            }, 
            0xA7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA3 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA4 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xA6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xE6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::And;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::and)
            }, 
            0xB7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB1 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB3 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB4 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB5 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xB6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xF6 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Or;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::or)
            }, 
            0xAF => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xA8 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xA9 => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAA => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAB => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAC => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAD => {
                self.cpu_ctx.instruction.in_type = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode = AddressMode::RR;
                self.cpu_ctx.instruction.function = Some(Cpu::xor_8);
            }, 
            0xAE => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::xor_8)
            }, 
            0xEE => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Xor;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::xor_8)
            }, 

            0xBF => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xB8 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xB9 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xBA => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xBB => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xBC => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xBD => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xBE => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RMr;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0xFE => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cp;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::cp)
            }, 
            0x3C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x04 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x0C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x14 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x1C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x24 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x2C => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x34 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::Mr;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x3D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::A;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x05 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::B;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x0D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::C;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x15 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::D;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x1D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::E;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x25 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::H;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x2D => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::L;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x35 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::Mr;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec);
            }, 
            0x09 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x19 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x29 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x39 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.register_2 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::RR;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0xE8 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Add;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::RD8;
                self.cpu_ctx.instruction.function   = Some(Cpu::add)
            }, 
            0x03 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x13 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x23 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x33 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Inc;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::inc)
            }, 
            0x0B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::BC;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec)
            }, 
            0x1B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::DE;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec)
            }, 
            0x2B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec)
            }, 
            0x3B => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Dec;
                self.cpu_ctx.instruction.register_1 = RegisterType::SP;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::dec)
            }, 
            0x27 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Daa;
                self.cpu_ctx.instruction.function   = Some(Cpu::daa)
            }, 
            0x2F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Cpl;
                self.cpu_ctx.instruction.function   = Some(Cpu::cpl)
            }, 
            0x3F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ccf;
                self.cpu_ctx.instruction.function   = Some(Cpu::ccf);
            }, 
            0x37 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Scf;
                self.cpu_ctx.instruction.function   = Some(Cpu::scf)
            }, 
            0x00 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Nop;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.function   = Some(Cpu::nop)
            }, 
            0x76 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Halt;
                self.cpu_ctx.instruction.function   = Some(Cpu::halt);
            }, 
            0x10 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Stop;
                self.cpu_ctx.instruction.function   = Some(Cpu::stop)
            }, 
            0xF3 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Di;
                self.cpu_ctx.instruction.function   = Some(Cpu::di);
            },
            0xFB => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ei;
                self.cpu_ctx.instruction.function   = Some(Cpu::ei)
            }, 
            0x07 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rlca;
                self.cpu_ctx.instruction.function   = Some(Cpu::rlca)
            }, 

            0x17 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rla;
                self.cpu_ctx.instruction.function   = Some(Cpu::rla);
            },

            0x0F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rrca;
                self.cpu_ctx.instruction.function   = Some(Cpu::rrca);
            }, 

            0x1F => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rra;
                self.cpu_ctx.instruction.function   = Some(Cpu::rra);
            },

            0xC3 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp);
            },

            0xC2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Nz;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp)
            }, 
            0xCA => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Z;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp)
            }, 
            0xD2 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Nc;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp)
            }, 
            0xDA => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::C;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp)
            }, 
            0xE9 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jp;
                self.cpu_ctx.instruction.register_1 = RegisterType::HL;
                self.cpu_ctx.instruction.mode       = AddressMode::R;
                self.cpu_ctx.instruction.function   = Some(Cpu::jp)
            }, 
            0x18 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jr;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.function   = Some(Cpu::jr)
            }, 
            0x20 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jr;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.condition  = ConditionType::Nz;
                self.cpu_ctx.instruction.function   = Some(Cpu::jr)
            }, 
            0x28 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jr;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.condition  = ConditionType::Z;
                self.cpu_ctx.instruction.function   = Some(Cpu::jr)
            }, 
            0x30 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jr;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.condition  = ConditionType::Nc;
                self.cpu_ctx.instruction.function   = Some(Cpu::jr)
            },
            0x38 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Jr;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D8;
                self.cpu_ctx.instruction.condition  = ConditionType::C;
                self.cpu_ctx.instruction.function   = Some(Cpu::jr)
            }, 

            0xCD => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Call;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.function   = Some(Cpu::call)
            }, 

            0xC4 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Call;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Nz;
                self.cpu_ctx.instruction.function   = Some(Cpu::call)
            }, 
            0xCC => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Call;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Z;
                self.cpu_ctx.instruction.function   = Some(Cpu::call)
            }, 
            0xD4 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Call;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::Nc;
                self.cpu_ctx.instruction.function   = Some(Cpu::call)
            }, 
            0xDC => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Call;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::D16;
                self.cpu_ctx.instruction.condition  = ConditionType::C;
                self.cpu_ctx.instruction.function   = Some(Cpu::call)
            }, 
            
            0xC7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x00;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xCF => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x08;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xD7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x10;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xDF => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x18;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xE7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x20;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xEF => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x28;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xF7 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x30;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            0xFF => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Rst;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.parmater   = 0x38;
                self.cpu_ctx.instruction.function   = Some(Cpu::rst)
            }, 
            
            0xC9 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ret;
                self.cpu_ctx.instruction.function   = Some(Cpu::ret)
            },

            0xC0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ret;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.condition  = ConditionType::Nz;
                self.cpu_ctx.instruction.function   = Some(Cpu::ret)
            }, 
            0xC8 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ret;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.condition  = ConditionType::Z;
                self.cpu_ctx.instruction.function   = Some(Cpu::ret)
            }, 
            0xD0 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ret;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.condition  = ConditionType::Nc;
                self.cpu_ctx.instruction.function   = Some(Cpu::ret)
            }, 
            0xD8 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Ret;
                self.cpu_ctx.instruction.register_1 = RegisterType::None;
                self.cpu_ctx.instruction.register_2 = RegisterType::None;
                self.cpu_ctx.instruction.mode       = AddressMode::Imp;
                self.cpu_ctx.instruction.condition  = ConditionType::C;
                self.cpu_ctx.instruction.function   = Some(Cpu::ret)
            }, 

            0xD9 => {
                self.cpu_ctx.instruction.in_type    = InstructionName::Reti;
                self.cpu_ctx.instruction.function   = Some(Cpu::reti)
            }, 
            _ => {}
        }
    }
}
