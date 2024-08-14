mod cart;
mod bus;
mod util;
mod cpu;
mod emu;
mod ppu;
mod ram;
use std::env;
mod ui;
use std::path::Path;
use emu::EmuContext;
use std::sync::{Arc, Mutex,atomic::AtomicBool};

fn main() {
    let emu = Arc::new(Mutex::new(EmuContext::default())); 
    let stop_flag = Arc::new(AtomicBool::new(false));

    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if a file name is provided
    if args.len() != 2 {
        eprintln!("Usage: cargo run <File Name>");
        return;
    }

    // Extract the file name
    let file_name = &args[1];

    // Construct the file path
    let rom_path = Path::new("../Roms").join(file_name);

    // Check if the file exists
    if !rom_path.exists() {
        eprintln!("Error: The file '{}' does not exist.", rom_path.display());
        return;
    }

    emu::EmuContext::emu_run(Arc::clone(&emu), rom_path.to_str().expect(""), Arc::clone(&stop_flag));
}
