use crate::cpu::register::RegisterType;

#[derive(Clone,Copy,Debug,Default)]
pub enum AddressMode {
    #[default]
    Imp,
    RD16,     
    RR,       
    MrR,      
    R,        
    RD8,      
    RMr,
    RHli,
    RHld,
    HliR,
    HldR,
    RA8,
    A8R,
    HlSpr,
    D16,
    D8,
    D16R,
    MrD8,
    Mr,
    A16R,
    RA16,
}

#[derive(Clone,Copy,Debug,Default)]
pub enum ConditionType {
    #[default]
    None,
    Nz,
    Z,
    Nc,
    C,
}

#[derive(Clone,Copy,Debug,Default)]
pub enum InstructionName {
    #[default]
    None,
    Nop,
    Ld,
    Inc,
    Dec,
    Rlca,
    Add,
    Rrca,
    Stop,
    Rla,
    Jr,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
    Pop,
    Jp,
    Push,
    Ret,
    Cb,
    Call,
    Reti,
    Ldh,
    Jphl,
    Di,
    Ei,
    Rst,
    Err,
    Rlc, 
    Rrc,
    Rl, 
    Rr,
    Sla, 
    Sra,
    Swap, 
    Srl,
    Bit, 
    Res, 
    Set
}

#[derive(Clone,Copy,Debug)]
pub struct Instruction {
    pub in_type: InstructionName,
    pub mode: AddressMode, 
    pub register_1: RegisterType, 
    pub register_2: RegisterType,
    pub condition: ConditionType, 
    pub parmater: u8,
} 

impl Default for Instruction {
    fn default() -> Self {
        Instruction {
            in_type: InstructionName::default(),
            mode: AddressMode::default(),
            register_1: RegisterType::default(),
            register_2: RegisterType::default(),
            condition: ConditionType::default(),
            parmater: 0,
        }
    }
}
