use crate::cpu::Cpu;

struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,
}

impl EmuContext {
   pub fn new() -> EmuContext {
        EmuContext {
            running: false,
            paused: true,
            ticks: 0
        } 
   } 
}

/*
 * Emu consists of:
 * Cart- load cart, read data, also write data
 * CPU- instructions, registers 
 * Address Bus- reading and writting to addresses (mem mapped)
 * PPU- Pixel processing unit
 * Timer- time stuff
 */

pub fn emu_get_context() {

}

pub fn delay(ms_int: u64) {
    let ms_time = std::time::Duration::from_millis(ms_int);
    std::thread::sleep(ms_time);
}

pub fn emu_run() -> i16 {
    let mut ctx: EmuContext = EmuContext::new();
    
    let mut cpu: Cpu = Cpu::new();
    cpu.init();
   
    while ctx.running {
       
        if ctx.paused {
            delay(10);
            continue;
        }

        if !cpu.step() {
            print!("CPU STOPPED");
            return -1;
        };

        ctx.ticks += 1;

    }
    0
}

