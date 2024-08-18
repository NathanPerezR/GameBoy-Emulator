use crate::cpu::Cpu;
use crate::bus::Bus;

#[derive(Clone, Copy, Debug)]
pub struct Debugger {
    dbg_msg: [u8; 1024],
    msg_size: usize,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            dbg_msg: [0 as u8; 1024],
            msg_size:  0,
        }
    }

    pub fn update(&mut self, mut cpu: Cpu, bus: &mut Bus) {
        if bus.read(0xFF02, cpu) == 0x81 {
            let c: u8 = bus.read(0xFF01, cpu) as u8;
            self.dbg_msg[self.msg_size] = c;
            self.msg_size += 1;
            bus.write(0xFF02, 0, &mut cpu);
        }
    }
    pub fn print(&self) {
        if self.msg_size > 0 {
            // Slice the dbg_msg buffer to only include the valid message part
            let dbg_msg_slice = &self.dbg_msg[0..self.msg_size];
            
            // Convert the slice to a String and print
            if let Ok(dbg_str) = std::str::from_utf8(dbg_msg_slice) {
                println!("DBG: {}", dbg_str);
            } else { 
                // Handle invalid UTF-8 (if necessary)
                println!("DBG: Invalid UTF-8 sequence detected.");
            }
        }
    }
    
}
