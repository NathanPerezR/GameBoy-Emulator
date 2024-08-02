use crate::cpu::{Cpu, Instruction, AddressMode};

impl Cpu {

    pub fn instruction_by_opcode(&mut self) -> Instruction {

        let mut to_return: Instruction = Instruction::default();

        match self.cpu_ctx.current_opcode {
            0x06 => {to_return},  
            0x0E => {to_return}, 
            0x16 => {to_return}, 
            0x1E => {to_return}, 
            0x26 => {to_return}, 
            0x2E => {to_return}, 
            0x7F => {to_return}, 
            0x78 => {to_return}, 
            0x79 => {to_return}, 
            0x7A => {to_return}, 
            0x7B => {to_return}, 
            0x7C => {to_return}, 
            0x7D => {to_return}, 
            0x7E => {to_return}, 
            0x40 => {to_return}, 
            0x41 => {to_return}, 
            0x42 => {to_return}, 
            0x43 => {to_return}, 
            0x44 => {to_return}, 
            0x45 => {to_return}, 
            0x46 => {to_return}, 
            0x48 => {to_return}, 
            0x49 => {to_return}, 
            0x4A => {to_return}, 
            0x4B => {to_return}, 
            0x4C => {to_return}, 
            0x4D => {to_return}, 
            0x4E => {to_return}, 
            0x50 => {to_return}, 
            0x51 => {to_return}, 
            0x52 => {to_return}, 
            0x53 => {to_return}, 
            0x54 => {to_return}, 
            0x55 => {to_return}, 
            0x56 => {to_return}, 
            0x58 => {to_return}, 
            0x59 => {to_return}, 
            0x5A => {to_return}, 
            0x5B => {to_return}, 
            0x5C => {to_return}, 
            0x5D => {to_return}, 
            0x5E => {to_return}, 
            0x60 => {to_return}, 
            0x61 => {to_return}, 
            0x62 => {to_return}, 
            0x63 => {to_return}, 
            0x64 => {to_return}, 
            0x65 => {to_return}, 
            0x66 => {to_return}, 
            0x68 => {to_return}, 
            0x69 => {to_return}, 
            0x6A => {to_return}, 
            0x6B => {to_return}, 
            0x6C => {to_return}, 
            0x6D => {to_return}, 
            0x6E => {to_return}, 
            0x70 => {to_return}, 
            0x71 => {to_return}, 
            0x72 => {to_return}, 
            0x73 => {to_return}, 
            0x74 => {to_return}, 
            0x75 => {to_return}, 
            0x36 => {to_return}, 
            0x0A => {to_return}, 
            0x1A => {to_return}, 
            0xFA => {to_return}, 
            0x3E => {to_return}, 
            0x47 => {to_return}, 
            0x4F => {to_return}, 
            0x57 => {to_return}, 
            0x5F => {to_return}, 
            0x67 => {to_return}, 
            0x6F => {to_return}, 
            0x02 => {to_return}, 
            0x12 => {to_return}, 
            0x77 => {to_return}, 
            0xEA => {to_return}, 
            0xF2 => {to_return}, 
            0xE2 => {to_return}, 
            0x3A => {to_return}, 
            0x32 => {to_return}, 
            0x2A => {to_return}, 
            0x22 => {to_return}, 
            0xE0 => {to_return}, 
            0xF0 => {to_return}, 
            0x01 => {to_return}, 
            0x11 => {to_return}, 
            0x21 => {to_return}, 
            0x31 => {to_return}, 
            0xF9 => {to_return}, 
            0xF8 => {to_return}, 
            0x08 => {to_return}, 
            0xF5 => {to_return}, 
            0xC5 => {to_return}, 
            0xD5 => {to_return}, 
            0xE5 => {to_return}, 
            0xF1 => {to_return}, 
            0xC1 => {to_return}, 
            0xD1 => {to_return}, 
            0xE1 => {to_return}, 
            0x87 => {to_return}, 
            0x80 => {to_return}, 
            0x81 => {to_return}, 
            0x82 => {to_return}, 
            0x83 => {to_return}, 
            0x84 => {to_return}, 
            0x85 => {to_return}, 
            0x86 => {to_return}, 
            0xC6 => {to_return}, 
            0x8F => {to_return}, 
            0x88 => {to_return}, 
            0x89 => {to_return}, 
            0x8A => {to_return}, 
            0x8B => {to_return}, 
            0x8C => {to_return}, 
            0x8D => {to_return}, 
            0x8E => {to_return}, 
            0xCE => {to_return}, 
            0x97 => {to_return}, 
            0x90 => {to_return}, 
            0x91 => {to_return}, 
            0x92 => {to_return}, 
            0x93 => {to_return}, 
            0x94 => {to_return}, 
            0x95 => {to_return}, 
            0x96 => {to_return}, 
            0xD6 => {to_return}, 
            0x9F => {to_return}, 
            0x98 => {to_return}, 
            0x99 => {to_return}, 
            0x9A => {to_return}, 
            0x9B => {to_return}, 
            0x9C => {to_return}, 
            0x9D => {to_return}, 
            0x9E => {to_return}, 
            0xA7 => {to_return}, 
            0xA0 => {to_return}, 
            0xA1 => {to_return}, 
            0xA2 => {to_return}, 
            0xA3 => {to_return}, 
            0xA4 => {to_return}, 
            0xA5 => {to_return}, 
            0xA6 => {to_return}, 
            0xE6 => {to_return}, 
            0xB7 => {to_return}, 
            0xB0 => {to_return}, 
            0xB1 => {to_return}, 
            0xB2 => {to_return}, 
            0xB3 => {to_return}, 
            0xB4 => {to_return}, 
            0xB5 => {to_return}, 
            0xB6 => {to_return}, 
            0xF6 => {to_return}, 
            0xAF => {to_return}, 
            0xA8 => {to_return}, 
            0xA9 => {to_return}, 
            0xAA => {to_return}, 
            0xAB => {to_return}, 
            0xAC => {to_return}, 
            0xAD => {to_return}, 
            0xAE => {to_return}, 
            0xEE => {to_return}, 
            0xBF => {to_return}, 
            0xB8 => {to_return}, 
            0xB9 => {to_return}, 
            0xBA => {to_return}, 
            0xBB => {to_return}, 
            0xBC => {to_return}, 
            0xBD => {to_return}, 
            0xBE => {to_return}, 
            0xFE => {to_return}, 
            0x3C => {to_return}, 
            0x04 => {to_return}, 
            0x0C => {to_return}, 
            0x14 => {to_return}, 
            0x1C => {to_return}, 
            0x24 => {to_return}, 
            0x2C => {to_return}, 
            0x34 => {to_return}, 
            0x3D => {to_return}, 
            0x05 => {to_return}, 
            0x0D => {to_return}, 
            0x15 => {to_return}, 
            0x1D => {to_return}, 
            0x25 => {to_return}, 
            0x2D => {to_return}, 
            0x35 => {to_return}, 
            0x09 => {to_return}, 
            0x19 => {to_return}, 
            0x29 => {to_return}, 
            0x39 => {to_return}, 
            0xE8 => {to_return}, 
            0x03 => {to_return}, 
            0x13 => {to_return}, 
            0x23 => {to_return}, 
            0x33 => {to_return}, 
            0x0B => {to_return}, 
            0x1B => {to_return}, 
            0x2B => {to_return}, 
            0x3B => {to_return}, 
            0x27 => {to_return}, 
            0x2F => {to_return}, 
            0x3F => {to_return}, 
            0x37 => {to_return}, 
            0x00 => {
                to_return.mode = AddressMode::AmImp;
                to_return
            }, 
            0x76 => {to_return}, 
            0x10 => {to_return}, 
            0xF3 => {to_return}, 
            0xFB => {to_return}, 
            0x07 => {to_return}, 
            0x17 => {to_return}, 
            0x0F => {to_return}, 
            0x1F => {to_return}, 
            0xC3 => {
                to_return.mode = AddressMode::AmD16;
                to_return
            }, 
            0xC2 => {to_return}, 
            0xCA => {to_return}, 
            0xD2 => {to_return}, 
            0xDA => {to_return}, 
            0xE9 => {to_return}, 
            0x18 => {to_return}, 
            0x20 => {to_return}, 
            0x28 => {to_return}, 
            0x30 => {to_return}, 
            0x38 => {to_return}, 
            0xCD => {to_return}, 
            0xC4 => {to_return}, 
            0xCC => {to_return}, 
            0xD4 => {to_return}, 
            0xDC => {to_return}, 
            0xC7 => {to_return}, 
            0xCF => {to_return}, 
            0xD7 => {to_return}, 
            0xDF => {to_return}, 
            0xE7 => {to_return}, 
            0xEF => {to_return}, 
            0xF7 => {to_return}, 
            0xFF => {to_return}, 
            0xC9 => {to_return}, 
            0xC0 => {to_return}, 
            0xC8 => {to_return}, 
            0xD0 => {to_return}, 
            0xD8 => {to_return}, 
            0xD9 => {to_return}, 
            _ => {to_return}
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
