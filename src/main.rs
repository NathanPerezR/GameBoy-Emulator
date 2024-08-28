mod cart;
mod bus;
mod util;
mod cpu;
mod emu;
mod ppu;
mod ram;
use std::env;
mod ui;
mod io;
mod timer;
mod interrupts;
mod dbg;
mod stack;
use std::path::Path;
use emu::EmuContext;

fn main() {
    let mut emu = EmuContext::default(); 

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

    EmuContext::emu_run(emu, rom_path.to_str().expect("")); 
}
