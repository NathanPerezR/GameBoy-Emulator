mod cart;
mod bus;
mod util;
mod cpu;
mod emu;

fn main() {
    let emu: emu::Emu = emu::Emu::default();
    emu.emu_run("/Users/nathanperez/repo/GameBoy-Emulator/Roms/dmg-acid2.gb");
}
