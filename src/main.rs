mod cart;
mod bus;
mod util;
mod cpu;
mod emu;

fn main() {
    let emu: emu::Emu = emu::Emu::default();
    emu.emu_run("../Roms/dmg-acid2.gb");
}
