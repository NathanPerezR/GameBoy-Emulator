extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use crate::EmuContext;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use crate::bus::Bus;

pub struct Ui {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    bus: Arc<Mutex<Bus>>,
}

impl Ui {
    pub fn new(screen_width: u32, screen_height: u32, bus: Arc<Mutex<Bus>>) -> Result<Self, String> {
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        let window = video_subsystem
            .window("Game Window", screen_width, screen_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Ui {
            sdl_context,
            canvas,
            bus,
        })
    }

    pub fn delay(&self, ms: u32) {
        ::std::thread::sleep(Duration::from_millis(ms.into()));
    }

    pub fn handle_events(&mut self, stop_flag: Arc<AtomicBool>) {
        let mut event_pump = self.sdl_context.event_pump().expect("Failed to get event pump");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Quit event received");
                },
                Event::Window { win_event, .. } if win_event == sdl2::event::WindowEvent::Close => {
                    println!("Window close event received");
                    stop_flag.store(true, Ordering::Relaxed);
                },
                _ => {}
            }
        }
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); 
        self.canvas.clear();
        self.canvas.present();
    }
}
