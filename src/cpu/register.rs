
#[derive(Clone, Copy, Debug, Default)]
pub struct RegisterData {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

pub enum RegisterType {
    A,
    B,
    C,
    D,
    E,
    F,
    AF,
    BC,
    DE,
    HL,
    PC,
    SP,
}



