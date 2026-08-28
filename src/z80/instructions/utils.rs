use crate::z80::instructions::InstructionError::UnsupportedRegister;
use crate::z80::instructions::constants::{A_REG, B_REG, C_REG, D_REG, E_REG, H_REG, L_REG};
use crate::z80::{Cpu, CycleError};

const LD_R_OPCODE_SHIFT: u8 = 3;

const R_REG_OPCODE_MASK: u8 = 0x07;

pub const fn r_reg_opcode(reg_r: u8, opcode_mask: u8) -> usize {
    (opcode_mask | (reg_r << LD_R_OPCODE_SHIFT)) as usize
}

pub fn set_r_register_from_opcode(opcode: u8, value: u8, cpu: &mut Cpu) -> Result<(), CycleError> {
    match r_reg_from_opcode(opcode) {
        A_REG => Ok(cpu.registers.set_a(value)),
        B_REG => Ok(cpu.registers.set_b(value)),
        C_REG => Ok(cpu.registers.set_c(value)),
        D_REG => Ok(cpu.registers.set_d(value)),
        E_REG => Ok(cpu.registers.set_e(value)),
        H_REG => Ok(cpu.registers.set_h(value)),
        L_REG => Ok(cpu.registers.set_l(value)),
        _ => Err(CycleError::UnsupportedInstruction(UnsupportedRegister)),
    }
}

pub const fn r_prime_reg_from_opcode(opcode: u8) -> u8 {
    opcode & R_REG_OPCODE_MASK
}

const fn r_reg_from_opcode(opcode: u8) -> u8 {
    (opcode >> LD_R_OPCODE_SHIFT) & R_REG_OPCODE_MASK
}
