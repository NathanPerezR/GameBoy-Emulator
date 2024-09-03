mod cart;
mod bus;
mod util;
mod cpu;
mod emu;
mod ppu;
mod ram;
mod ui;
mod io;
mod timer;
mod interrupts;
mod dbg;
mod stack;
mod dma;
use emu::EmuContext;
use std::path::Path;
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    // Initialize EmuContext
    let emu = Arc::new(Mutex::new(EmuContext::default())); 

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
    let rom_path = Path::new("../Testroms/").join(file_name);

    // Check if the file exists
    if !rom_path.exists() {
        eprintln!("Error: The file '{}' does not exist.", rom_path.display());
        return;
    }

    // Convert the path to a string slice
    let rom_path_str = rom_path.to_str().expect("Failed to convert path to string");

    // Call the emu_run function with shared ownership
    let result = EmuContext::emu_run(Arc::clone(&emu), rom_path_str);

    if result != 0 {
        eprintln!("An error occurred during emulation.");
    }
}




