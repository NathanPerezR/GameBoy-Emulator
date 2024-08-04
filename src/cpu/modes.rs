use crate::cpu::{Cpu, Instruction};
use crate::cpu::structs::{InstructionName, ConditionType, AddressMode};

impl Cpu {

    pub fn instruction_by_opcode(&mut self) -> Instruction {

        let mut to_return: Instruction = Instruction::default();

        match self.cpu_ctx.current_opcode {
            0x06 => {},  
            0x0E => {}, 
            0x16 => {}, 
            0x1E => {}, 
            0x26 => {}, 
            0x2E => {}, 
            0x7F => {}, 
            0x78 => {}, 
            0x79 => {}, 
            0x7A => {}, 
            0x7B => {}, 
            0x7C => {}, 
            0x7D => {}, 
            0x7E => {}, 
            0x40 => {}, 
            0x41 => {}, 
            0x42 => {}, 
            0x43 => {}, 
            0x44 => {}, 
            0x45 => {}, 
            0x46 => {}, 
            0x48 => {}, 
            0x49 => {}, 
            0x4A => {}, 
            0x4B => {}, 
            0x4C => {}, 
            0x4D => {}, 
            0x4E => {}, 
            0x50 => {}, 
            0x51 => {}, 
            0x52 => {}, 
            0x53 => {}, 
            0x54 => {}, 
            0x55 => {}, 
            0x56 => {}, 
            0x58 => {}, 
            0x59 => {}, 
            0x5A => {}, 
            0x5B => {}, 
            0x5C => {}, 
            0x5D => {}, 
            0x5E => {}, 
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
            0x47 => {}, 
            0x4F => {}, 
            0x57 => {}, 
            0x5F => {}, 
            0x67 => {}, 
            0x6F => {}, 
            0x02 => {}, 
            0x12 => {}, 
            0x77 => {}, 
            0xEA => {}, 
            0xF2 => {}, 
            0xE2 => {}, 
            0x3A => {}, 
            0x32 => {}, 
            0x2A => {}, 
            0x22 => {}, 
            0xE0 => {}, 
            0xF0 => {}, 
            
            // 16 bit loads
            0x01 => {
                to_return.in_type = InstructionName::Ld;
                to_return.mode = AddressMode::D16;
            }, 
            0x11 => {
                to_return.in_type = InstructionName::Ld;
                to_return.mode = AddressMode::D16;
            }, 
            0x21 => {
                to_return.in_type = InstructionName::Ld;
                to_return.mode = AddressMode::D16;
            }, 
            0x31 => {
                to_return.in_type = InstructionName::Ld;
                to_return.mode = AddressMode::D16;
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

            // XOR n 
            0xAF => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xA8 => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xA9 => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xAA => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xAB => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xAC => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xAD => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xAE => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 
            0xEE => {
                to_return.in_type = InstructionName::Xor;
                to_return.mode = AddressMode::R;
            }, 

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
                to_return.in_type = InstructionName::Nop;
                to_return.mode = AddressMode::Imp;
            }, 

            0x76 => {}, 
            0x10 => {}, 

            0xF3 => {
                to_return.in_type = InstructionName::Di;
            },

            0xFB => {}, 
            0x07 => {}, 
            0x17 => {}, 
            0x0F => {}, 
            0x1F => {},

            0xC3 => {
                to_return.in_type = InstructionName::Jp;
                to_return.mode = AddressMode::D16;
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

        to_return
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
