use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::ui::Ui;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::time::Duration;
use std::thread;

pub struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,

    pub cpu: Cpu,
    pub bus: Arc<Mutex<Bus>>,
}

impl Default for EmuContext {
   fn default() -> Self {
        EmuContext {
            running: true,
            paused: false,
            ticks: 0,
            cpu: Cpu::default(),
            bus: Arc::new(Mutex::new(Bus::default())),
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
            {
                let mut bus = self.bus.lock().unwrap();
                if !self.cpu.step(&mut bus) {
                    println!("CPU Stopped");
                    return;
                }
            }
            self.ticks += 1;
        }
    }

    pub fn emu_run(emu_context: Arc<Mutex<EmuContext>>, rom_path: &str, stop_flag: Arc<AtomicBool>) -> i16 {
        let emu = emu_context.lock().unwrap();

        {
            let mut bus = emu.bus.lock().unwrap();
            if !bus.cart.cart_load(rom_path) {
                println!("Failed to load ROM file: {}", rom_path);
                return -1;
            }
        } // drop lock here

        let bus_arc = Arc::clone(&emu.bus);
        let stop_flag_clone = Arc::clone(&stop_flag);

        drop(emu); 

        let mut ui = Ui::new(800, 800, bus_arc).expect("Failed to initialize UI");

        let cpu_thread = thread::spawn(move || {
            let mut emu = emu_context.lock().unwrap();
            while !stop_flag_clone.load(Ordering::Relaxed) {
                emu.cpu_run();
            }
        });

        while !stop_flag.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(1000));
            ui.handle_events(stop_flag.clone());
            ui.render();
        }

        stop_flag.store(true, Ordering::Relaxed);
        cpu_thread.join().unwrap();

        0
    }


}
