use crate::dbg::Debugger;
use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::ui::{UI, SCREEN_WIDTH, SCREEN_HEIGHT};
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,
    pub die: bool, 

    pub cpu: Arc<Mutex<Cpu>>,
    pub bus: Arc<Mutex<Bus>>,
    pub dbg: Arc<Mutex<Debugger>>,
}

impl Default for EmuContext {
   fn default() -> Self {
        EmuContext {
            running: true,
            paused: false,
            ticks: 0,
            cpu: Arc::new(Mutex::new(Cpu::default())),
            bus: Arc::new(Mutex::new(Bus::default())),
            dbg: Arc::new(Mutex::new(Debugger::new())),
            die: false,
        } 
   } 
}

impl EmuContext {

    pub fn delay(&self, ms_int: u64) {
        let ms_time = Duration::from_millis(ms_int);
        std::thread::sleep(ms_time);
    }

    pub fn cpu_step(&self) {
        let mut cpu = self.cpu.lock().unwrap();
        let mut bus = self.bus.lock().unwrap();
        let mut dbg = self.dbg.lock().unwrap();
        cpu.step(&mut bus, &mut dbg);
    }


    pub fn emu_run(emu_context: Arc<Mutex<EmuContext>>, rom_path: &str) -> i16 {

        {
            // Lock emu_context and then bus
            let emu_context = emu_context.lock().unwrap();
            let mut bus = emu_context.bus.lock().unwrap();

            // Attempt to load the ROM
            if !bus.cart.cart_load(rom_path) {
                println!("Failed to load ROM file: {}", rom_path);
                return -1;
            }

        } // `bus` and `emu_context` are unlocked here

        let mut ui = UI::new().expect("Failed to initialize UI");

        let stop_flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let stop_flag_clone = Arc::clone(&stop_flag);
        let emu_context_clone = Arc::clone(&emu_context);

        let cpu_thread = thread::spawn(move || {
            while !stop_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                emu_context_clone.lock().unwrap().cpu_step();
                thread::sleep(Duration::from_micros(1));
            }
        });

        'running: loop {
            for event in ui.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        break 'running;
                    },
                    //TODO: add more event handling
                    _ => {}
                }
            }

            // Update emulator state and frame buffer
            {
                let emu = emu_context.lock().unwrap();
                let mut bus = emu.bus.lock().unwrap();
                let cpu = emu.cpu.lock().unwrap();

                ui.update_dbg_window(&mut bus, &cpu);
            }



            // Control frame rate
            ui.delay(16);
        }

        stop_flag.store(true, std::sync::atomic::Ordering::Relaxed);
        cpu_thread.join().unwrap();
        0
    }

}
