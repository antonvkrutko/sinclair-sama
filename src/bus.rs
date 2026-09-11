use crate::bus::BusError::{AccessError, AddressOutOfRange};
use crate::bus::Region::{Ram, Rom};
use log::debug;

const ROM_SIZE: usize = 1024;
const RAM_SIZE: usize = 1024;

///
/// Z80 assumes that the memory structure is Linear. ROM -> RAM.
///
pub struct Bus {
    rom: [u8; ROM_SIZE], // 1kb
    ram: [u8; RAM_SIZE], // 1kb
}

#[derive(Debug)]
pub enum BusError {
    AddressOutOfRange,
    AccessError,
}

enum Region {
    Rom,
    Ram,
}

pub trait Readable {
    fn read(&self, addr: u16) -> Result<u8, BusError>;
}

pub trait Writable {
    fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError>;
}

impl Readable for Bus {
    fn read(&self, addr: u16) -> Result<u8, BusError> {
        let addr = usize::from(addr);
        let (addr, region) = self.translate_address(addr)?;
        Ok(match region {
            Rom => self.rom[addr],
            Ram => self.ram[addr],
        })
    }
}

impl Writable for Bus {
    fn write(&mut self, addr: u16, data: u8) -> Result<(), BusError> {
        let addr = usize::from(addr);
        let (addr, region) = self.translate_address(addr)?;
        match region {
            Rom => Err(AccessError),
            Ram => {
                self.ram[addr] = data;
                Ok(())
            }
        }
    }
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            rom: [0; ROM_SIZE],
            ram: [0; RAM_SIZE],
        }
    }

    fn translate_address(&self, addr: usize) -> Result<(usize, Region), BusError> {
        if addr >= ROM_SIZE + RAM_SIZE {
            return Err(AddressOutOfRange);
        }

        if addr < ROM_SIZE {
            return Ok((addr, Rom));
        }

        let ram_addr = addr - ROM_SIZE;
        Ok((ram_addr, Ram))
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
            debug!("{address_line_string}");
        }
    }
}
