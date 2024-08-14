mod codes;
use std::fs::File;
use std::io::{self, Read};


#[derive(Default,Debug)]
struct CartHeader {
    entry: [u8; 4],
    logo: Vec<u8>,
    title: String,
    new_lic_code: u16,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16,
}


#[derive(Default)]
struct CartContext {
    filename: String,
    rom_size: usize,
    rom_data: Vec<u8>
}

#[derive(Default)]
pub struct Cart {
    header: CartHeader,
    cart_ctx: CartContext,
}

impl Cart {

    pub fn cart_load(&mut self, path: &str) -> bool { 
        self.cart_ctx.filename = path.to_string();
        self.cart_ctx.rom_data = read_file_to_vec(path).expect("Failed to load file");

        self.cart_ctx.rom_size = self.cart_ctx.rom_data.len();

        self.save_cart_data();

        self.print_cart_info();

        // checksum
        let mut checksum: u16 = 0;
        for address in 0x0134..=0x14C {
            checksum = checksum.wrapping_sub(self.cart_ctx.rom_data[address].into()).wrapping_sub(1);
        }
        
        let mut passed_check_sum: &str = "FAILED";
        if (checksum & 0xFF) == (self.header.checksum).into() {
            passed_check_sum = "PASSED";
        } 

        println!("\t Check Sum: {}", passed_check_sum);
    
        true
    }

    pub fn cart_read(&self, address: u16) -> u8 {
        self.cart_ctx.rom_data[address as usize]
    }

    pub fn cart_write(&mut self, address: u16, value: u8) -> bool {
        false
    }

    fn save_cart_data(&mut self) {
        let data = &self.cart_ctx.rom_data;
        
        // values below are from the gameboy pandocs 
        self.header.entry.copy_from_slice(&data[0x100..0x104]);
        self.header.logo = data[0x104..0x0132].to_vec();
        self.header.title = String::from_utf8_lossy(&data[0x134..0x144]).trim_end().to_string();
        self.header.new_lic_code = u16::from_le_bytes([data[0x144], data[0x145]]);
        self.header.old_lic_code = data[0x014B];
        self.header.sgb_flag = data[0x0143];
        self.header.cart_type = data[0x0147];
        self.header.rom_size = data[0x0148];
        self.header.ram_size = data[0x0149];
        self.header.dest_code = data[0x014A];
        self.header.version = data[0x014C];
        self.header.checksum = data[0x014D];
        self.header.global_checksum = u16::from_le_bytes([data[0x14E], data[0x14F]]);
    }

    fn print_cart_info(&self) {
        println!("Cartridge Loaded:");
        println!("\t Title    : {}", self.header.title);
        println!("\t Type     : {:02X}: {}", self.header.cart_type, self.print_rom_type());
        println!("\t ROM Size : {} KB", 32 << self.header.rom_size);
        println!("\t RAM Size : {:02X}", self.header.ram_size);
        println!("\t LIC Code : {:02X}: {}", self.header.new_lic_code, self.print_lic_code());
        println!("\t ROM Vers : {:02X}", self.header.version);
    }

    fn print_lic_code(&self) -> &str {
        if self.header.old_lic_code == 0x33 {
            return self.new_lic_code_lookup()
        }
        self.old_lic_code_lookup()
    }

    fn print_rom_type(&self) -> &str {
        self.rom_type_lookup()
    }
}

fn read_file_to_vec(file_path: &str) -> Result<Vec<u8>, io::Error> {

    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}


