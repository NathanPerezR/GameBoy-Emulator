mod cart;
mod bus;
mod util;
mod cpu;
mod emu;
mod ppu;
mod ram;
use std::env;
use std::path::Path;

fn main() {
    let emu: emu::Emu = emu::Emu::default();

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

    emu.emu_run(rom_path.to_str().expect(""));
}
