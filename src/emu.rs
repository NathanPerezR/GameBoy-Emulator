use crate::dbg::Debugger;
use crate::{cpu::Cpu};
use crate::bus::Bus;
use crate::ui::Ui;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,

    pub cpu: Cpu,
    pub bus: Bus,
    pub dbg: Debugger
}

impl Default for EmuContext {
   fn default() -> Self {
        EmuContext {
            running: true,
            paused: false,
            ticks: 0,
            cpu: Cpu::default(),
            bus: Bus::default(),
            dbg: Debugger::new()
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

            let step_result = {

                self.cpu.step(&mut self.bus, &mut self.dbg)
            };

            if !step_result {
                println!("CPU Stopped");
                return;
            }

            self.ticks += 1;
        }
    }

    pub fn emu_run(mut self, rom_path: &str) -> i16 {

        if !self.bus.cart.cart_load(rom_path) {
            println!("Failed to load ROM file: {}", rom_path);
            return -1;
        }

        let mut ui = Ui::new(800, 800).expect("Failed to initialize UI");

        thread::spawn(move|| {
            self.cpu_run();
        });

        loop {
            ui.handle_events();
            ui.render();
            thread::sleep(Duration::from_millis(100));
        }
    }
}
