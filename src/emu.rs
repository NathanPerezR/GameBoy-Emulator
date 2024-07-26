use crate::cpu::Cpu;
use crate::cart::Cart;

struct EmuContext {
    pub running: bool,
    pub paused: bool,
    pub ticks: u64,
}

impl Default for EmuContext {
   fn default() -> EmuContext {
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

#[derive(Default)]
pub struct Emu {
    emu_ctx: EmuContext,
}

impl Emu {

    pub fn delay(&self, ms_int: u64) {
        let ms_time = std::time::Duration::from_millis(ms_int);
        std::thread::sleep(ms_time);
    }

    pub fn emu_run(&self, rom_path: &str) -> i16 {

        let mut ctx: EmuContext = EmuContext::default();
        
        let mut cpu: Cpu = Cpu::new();
        cpu.init();

        let mut cart: Cart = Cart::default();
        cart.cart_load(rom_path);
       
        while ctx.running {
           
            if ctx.paused {
                self.delay(10);
                continue;
            }

            if !cpu.step(cart) {
                print!("CPU STOPPED");
                return -1;
            };

            ctx.ticks += 1;

        }
        0
    }
}
