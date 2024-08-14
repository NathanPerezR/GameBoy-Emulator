use crate::cpu::Cpu;
use crate::bus::Bus;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::thread;

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
        let ms_time = Duration::from_millis(ms_int);
        std::thread::sleep(ms_time);
    }


    pub fn cpu_run(&mut self) {
        self.running = true;
        self.paused = false;
        self.ticks = 0;

        while self.running {
            if self.paused {
                self.delay(10);
                continue;
            }
            if !self.cpu.step(&mut self.bus) {
                println!("CPU Stopped");
                return;
            }
            self.ticks += 1;
        }
    }

    pub fn emu_run(emu_context: Arc<Mutex<EmuContext>>, rom_path: &str, stop_flag: Arc<AtomicBool>) -> i16 {
        let mut emu = emu_context.lock().unwrap();
        
        if !emu.bus.cart.cart_load(rom_path) {
            println!("Failed to load ROM file: {}", rom_path);
            return -1;
        }

        let stop_flag_clone = Arc::clone(&stop_flag);
        let emu_context_clone = Arc::clone(&emu_context);

        let cpu_thread = thread::spawn(move || {
            let mut emu = emu_context_clone.lock().unwrap();
            while !stop_flag_clone.load(Ordering::Relaxed) {
                emu.cpu_run();
            }
        });

        drop(emu); // Release the lock on emu_context before the main thread continues

        while !stop_flag.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(1000));
            // UI HERE
        }

        stop_flag.store(true, Ordering::Relaxed);
        cpu_thread.join().unwrap();

        0
    }

}
