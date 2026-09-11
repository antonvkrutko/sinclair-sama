use crate::z80::instructions::InstructionError::UnsupportedRegister;

#[derive(Debug, Copy, Clone)]
pub enum Reg {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub const REGS: [Reg; 7] = [Reg::A, Reg::B, Reg::C, Reg::D, Reg::E, Reg::H, Reg::L];

impl TryFrom<u8> for Reg {
    type Error = crate::z80::InstructionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            A_REG => Ok(Reg::A),
            B_REG => Ok(Reg::B),
            C_REG => Ok(Reg::C),
            D_REG => Ok(Reg::D),
            E_REG => Ok(Reg::E),
            H_REG => Ok(Reg::H),
            L_REG => Ok(Reg::L),
            _ => Err(UnsupportedRegister),
        }
    }
}

pub const fn reg_code_from(value: Reg) -> u8 {
    match value {
        Reg::A => A_REG,
        Reg::B => B_REG,
        Reg::C => C_REG,
        Reg::D => D_REG,
        Reg::E => E_REG,
        Reg::H => H_REG,
        Reg::L => L_REG,
    }
}

const A_REG: u8 = 0x07;
const B_REG: u8 = 0x00;
const C_REG: u8 = 0x01;
const D_REG: u8 = 0x02;
const E_REG: u8 = 0x03;
const H_REG: u8 = 0x04;
const L_REG: u8 = 0x05;
