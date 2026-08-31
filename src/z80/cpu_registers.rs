use std::fmt::{Display, Formatter};
use log::debug;

pub struct CpuRegisters {
    // general purpose registers
    bc: u16,
    de: u16,
    hl: u16,

    bc_: u16,
    de_: u16,
    hl_: u16,

    // special purpose registers
    pc: u16,
    sp: u16,

    // accumulators
    a: u8,
    a_: u8,

    // flags
    f: u8,
    f_: u8,

    ix: u16,
    iy: u16,
}

impl CpuRegisters {
    pub fn new() -> Self {
        CpuRegisters {
            bc: 0,
            de: 0,
            hl: 0,
            bc_: 0,
            de_: 0,
            hl_: 0,
            pc: 0,
            sp: 0,
            a: 0,
            a_: 0,
            f: 0,
            f_: 0,
            ix: 0,
            iy: 0,
        }
    }

    pub fn current_pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value
    }

    pub fn get_b(&self) -> u8 {
        self.bc.to_be_bytes()[0]
    }

    pub fn set_b(&mut self, value: u8) {
        let b_value = u16::from(value) << u8::BITS;
        self.bc = (self.bc & 0x00FF) | b_value
    }

    pub fn get_c(&self) -> u8 {
        self.bc.to_be_bytes()[1]
    }

    pub fn set_c(&mut self, value: u8) {
        let c_value = u16::from(value);
        self.bc = (self.bc & 0xFF00) | c_value
    }

    pub fn get_d(&self) -> u8 {
        self.de.to_be_bytes()[0]
    }

    pub fn set_d(&mut self, value: u8) {
        let d_value = u16::from(value) << u8::BITS;
        self.de = (self.de & 0x00FF) | d_value
    }

    pub fn get_e(&self) -> u8 {
        self.de.to_be_bytes()[1]
    }

    pub fn set_e(&mut self, value: u8) {
        let e_value = u16::from(value);
        self.de = (self.de & 0xFF00) | e_value
    }

    pub fn get_h(&self) -> u8 {
        self.hl.to_be_bytes()[0]
    }

    pub fn set_h(&mut self, value: u8) {
        let h_value = u16::from(value) << u8::BITS;
        self.hl = (self.hl & 0x00FF) | h_value
    }

    pub fn get_l(&self) -> u8 {
        self.hl.to_be_bytes()[1]
    }

    pub fn set_l(&mut self, value: u8) {
        let l_value = u16::from(value);
        self.hl = (self.hl & 0xFF00) | l_value
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn get_ix(&self) -> u16 {
        self.ix
    }

    pub fn dump(&self) {
        debug!("CpuRegisters dump state:\n{self}");
    }
}

impl Display for CpuRegisters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let bc_bytes: [u8; 2] = self.bc.to_be_bytes();
        let de_bytes: [u8; 2] = self.de.to_be_bytes();
        let hl_bytes: [u8; 2] = self.hl.to_be_bytes();

        write!(
            f,
            "PC: 0x{:04X}\nA: 0x{:02X}\nBC: 0x{:04X} (B: 0x{:02X}, C: 0x{:02X})\n\
            DE: 0x{:04X} (D: 0x{:02X}, E: 0x{:02X})\n\
            HL: 0x{:04X} (H: 0x{:02X}, L: 0x{:02X})",
            self.pc,
            self.a,
            self.bc,
            bc_bytes[0],
            bc_bytes[1],
            self.de,
            de_bytes[0],
            de_bytes[1],
            self.hl,
            hl_bytes[0],
            hl_bytes[1]
        )
    }
}
