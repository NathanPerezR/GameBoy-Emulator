use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::surface::Surface;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::rect::Rect;
use sdl2::render::TextureAccess;
use sdl2::pixels::Color;
use std::time::Duration;
use std::cell::RefCell;
use crate::bus::Bus;
use crate::cpu::Cpu;

pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 768;
const SCALE: u32 = 4;

const TILE_COLORS: [Color; 4] = [
    Color::RGBA(255, 255, 255, 255),
    Color::RGBA(170, 170, 170, 255),
    Color::RGBA(85, 85, 85, 255),
    Color::RGBA(0, 0, 0, 255),
];

pub struct UI {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub event_pump: EventPump,
    pub sdl_renderer: Canvas<Window>,
    pub sdl_texture_creator: TextureCreator<WindowContext>,
    pub sdl_debug_renderer: RefCell<Canvas<Window>>,
    pub sdl_debug_texture_creator: TextureCreator<WindowContext>,
    pub debug_screen: Surface<'static>,
}

impl UI {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        let (sdl_renderer, sdl_texture_creator) = UI::create_renderer(&video_subsystem)?;
        let (sdl_debug_renderer, sdl_debug_texture_creator) = UI::create_debug_renderer(&video_subsystem)?;
        let debug_screen = UI::create_debug_surface()?;

        Ok(UI {
            sdl_context,
            video_subsystem,
            event_pump,
            sdl_renderer,
            sdl_texture_creator,
            sdl_debug_renderer: sdl_debug_renderer.into(),
            sdl_debug_texture_creator,
            debug_screen,
        })
    }

    fn create_renderer(video_subsystem: &VideoSubsystem) -> Result<(Canvas<Window>, TextureCreator<WindowContext>), String> {
        let sdl_window = video_subsystem
            .window("Gameboy Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = sdl_window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok((canvas, texture_creator))
    }

    fn create_debug_renderer(video_subsystem: &VideoSubsystem) -> Result<(Canvas<Window>, TextureCreator<WindowContext>), String> {
        let sdl_debug_window = video_subsystem
            .window("Debug Window", 16 * 8 * SCALE, 32 * 8 * SCALE)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = sdl_debug_window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok((canvas, texture_creator))
    }

    fn create_debug_surface() -> Result<Surface<'static>, String> {
        Surface::new(
            (16 * 8 * SCALE) + (16 * SCALE) ,
            (32 * 8 * SCALE) + (64 * SCALE) ,
            PixelFormatEnum::ARGB8888,
        ).map_err(|e| e.to_string())
    }

    pub fn create_texture(&self) -> Result<Texture, String> {
        self.sdl_texture_creator.create_texture(
            PixelFormatEnum::ARGB8888,
            TextureAccess::Streaming,
            SCREEN_WIDTH,
            SCREEN_HEIGHT
        ).map_err(|e| e.to_string())
    }

    pub fn create_debug_texture(&self) -> Result<Texture, String> {
        self.sdl_debug_texture_creator.create_texture(
            PixelFormatEnum::ARGB8888,
            TextureAccess::Streaming,
            self.debug_screen.width(),
            self.debug_screen.height()
        ).map_err(|e| e.to_string())
    }

    pub fn delay(&self, ms: u32) {
        ::std::thread::sleep(Duration::from_millis(ms as u64));
    }

    pub fn display_tile(&mut self, bus: &mut Bus, cpu: &Cpu, start_location: u16, tile_num: u16, x: i32, y: i32) {
        let tile_size = 16;
        for tile_y in (0..tile_size).step_by(2) {
            let b1 = bus.read(start_location + (tile_num * tile_size) + tile_y, cpu);
            let b2 = bus.read(start_location + (tile_num * tile_size) + tile_y + 1, cpu);

            for bit in (0..8).rev() {
                let hi = (b1 & (1 << bit)) >> bit;
                let lo = (b2 & (1 << bit)) >> bit;
                let color_index = (hi << 1) | lo;
                let rc = Rect::new(
                    (x as u32 + ((7 - bit) * SCALE)).try_into().unwrap(),
                    (y as u32 + (tile_y / 2 * SCALE as u16) as u32).try_into().unwrap(),
                    SCALE,
                    SCALE,
                );
                                                                                          
                self.debug_screen.fill_rect(rc, TILE_COLORS[color_index as usize]).unwrap();
            }
        }
    }
                                                                                      
    pub fn update_dbg_window(&mut self, bus: &mut Bus, cpu: &Cpu) {

        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_num = 0;

        let rc = Rect::new(0, 0, self.debug_screen.width(), self.debug_screen.height());
        self.debug_screen.fill_rect(rc, Color::RGBA(17, 17, 17, 255)).unwrap();

        let addr = 0x8000;
        for _ in 0..24 {
            for _ in 0..16 {
                self.display_tile(bus, cpu, addr, tile_num, x_draw, y_draw);
                x_draw += 8 * SCALE as i32;
                tile_num += 1;
            }
            x_draw = 0;
            y_draw += 8 * SCALE as i32;
        }

        let texture_creator = &self.sdl_debug_texture_creator;
        let debug_texture = texture_creator.create_texture_from_surface(&self.debug_screen).unwrap();

        let mut renderer = self.sdl_debug_renderer.borrow_mut();
        renderer.clear();
        renderer.copy(&debug_texture, None, None).expect("Failed to copy debug texture");
        renderer.present();
    }
}
