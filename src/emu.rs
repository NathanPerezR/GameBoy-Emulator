use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,

    pub cpu: Cpu,
    pub bus: Bus, 
}

impl Default for EmuContext {
   fn default() -> Self {
        EmuContext {
            running: true,
            paused: false,
            ticks: 0,
            cpu: Cpu::default(),
            bus: Bus::default(),
        } 
   } 
}

impl EmuContext {

    pub fn delay(&self, ms_int: u64) {
        let ms_time = std::time::Duration::from_millis(ms_int);
        std::thread::sleep(ms_time);
    }

    pub fn emu_run(&mut self, rom_path: &str) -> i16 {

        self.bus.cart.cart_load(rom_path);
        while self.running {
           
            if self.paused {
                self.delay(10);
                continue;
            }

            if !self.cpu.step(&mut self.bus) {
                print!("CPU STOPPED");
                return -1;
            };

            self.ticks += 1;

        }
        0
    }
}
