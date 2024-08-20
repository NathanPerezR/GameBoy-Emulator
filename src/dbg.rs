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
            dbg_msg: [0; 1024],
            msg_size: 0,
        }
    }

    pub fn update(&mut self, cpu: &mut Cpu, bus: &mut Bus) {
        if bus.read(0xFF02, cpu) == 0x81 {
            let c: u8 = bus.read(0xFF01, cpu);

            // Ensure we do not exceed the buffer size
            if self.msg_size < self.dbg_msg.len() {
                self.dbg_msg[self.msg_size] = c;
                self.msg_size = self.msg_size.wrapping_add(1);
            }

            bus.write(0xFF02, 0, cpu);
        }
    }

    pub fn print(&self) {
        // Print the message as a C-style string
        if self.msg_size > 0 {
            // Convert to a slice and create a null-terminated string
            let dbg_msg_slice = &self.dbg_msg[0..self.msg_size];
            if let Ok(dbg_str) = std::str::from_utf8(dbg_msg_slice) {
                println!("DBG: {}", dbg_str);
            } else {
                println!("DBG: Invalid UTF-8 sequence detected.");
            }
        }
    }
}

