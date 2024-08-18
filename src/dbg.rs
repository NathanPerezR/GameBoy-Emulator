use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct Debugger {
    dbg_msg: Vec<char>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            dbg_msg: Vec::with_capacity(1024),
        }
    }

    pub fn update(&mut self, mut cpu: Cpu, bus: &mut Bus) {
        if bus.read(0xFF02, cpu) == 0x81 {
            let c = bus.read(0xFF01, cpu) as char;
            self.dbg_msg.push(c);
            bus.write(0xFF02, 0, &mut cpu);
        }
    }

    pub fn print(&self) {
        if !self.dbg_msg.is_empty() {
            let dbg_str: String = self.dbg_msg.iter().collect();
            println!("DBG: {}", dbg_str);
        }
    }
}
