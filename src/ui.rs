extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

pub struct Ui {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
}

impl Ui {
    pub fn new(screen_width: u32, screen_height: u32) -> Result<Self, String> {
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
        })
    }

    pub fn delay(&self, ms: u32) {
        ::std::thread::sleep(Duration::from_millis(ms.into()));
    }

    pub fn handle_events(&self) {
        let mut event_pump = self.sdl_context.event_pump().expect("Failed to get event pump");
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    // Set flag or perform action to exit the loop
                    println!("Quit event received");
                    // This should break or exit your main loop
                },
                Event::Window { win_event, .. } if win_event == sdl2::event::WindowEvent::Close => {
                    // Handle window close event
                    println!("Window close event received");
                    // This should break or exit your main loop
                },
                _ => {}
            }
        }
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set background color
        self.canvas.clear();
        // Drawing code goes here
        self.canvas.present();
    }
}

fn main() {
    const SCREEN_WIDTH: u32 = 800;
    const SCREEN_HEIGHT: u32 = 600;

    let mut ui = Ui::new(SCREEN_WIDTH, SCREEN_HEIGHT).expect("Failed to initialize UI");

    'running: loop {
        ui.handle_events();
        ui.render();
        ui.delay(16); // Roughly 60 FPS
    }
}
