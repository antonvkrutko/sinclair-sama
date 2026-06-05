use std::fmt::format;

pub struct Bus {
    rom: [u8; 1024], // 1kb
}

#[derive(Debug)]
pub enum BusError {
    AddressOutOfRange
}

impl Bus {
    pub fn new() -> Self {
        Bus { rom: [0; 1024] }
    }

    pub fn read(&self, addr: u16) -> Result<u8, BusError> {
        let addr = usize::from(addr);
        if addr < self.rom.len() {
            Ok(self.rom[addr])
        } else {
            Err(BusError::AddressOutOfRange)
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }

    pub fn load_rom_raw(&mut self, data: &[u8]) {
        if data.len() > self.rom.len() {
            panic!("Failed to load raw rom. Data is too big")
        }

        let rom_slice_to_fill = &mut self.rom[..data.len()];
        rom_slice_to_fill.copy_from_slice(data)
    }

    pub fn dump_rom(&self) {
        let bytes_in_row = 8;
        let address_lines = self.rom.len() / bytes_in_row;
        for address_line in 0..address_lines {
            let mut address_line_string = format!("0x{:04X}:", address_line * bytes_in_row);
            for byte in 0..8 {
                let byte_to_print = self.rom[address_line * bytes_in_row + byte];
                address_line_string.push_str(&format!(" {:02X}", byte_to_print));
            }
            println!("{address_line_string}");
        }
    }
}
