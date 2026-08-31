use crate::z80::Cpu;
use crate::z80::instructions::InstructionError;
use crate::z80::instructions::constants::{Reg, reg_code_from};

const LD_R_OPCODE_SHIFT: u8 = 3;

const R_REG_OPCODE_MASK: u8 = 0x07;

pub const fn r_reg_opcode(reg: &Reg, opcode_mask: u8) -> usize {
    let reg_r = reg_code_from(reg);
    (opcode_mask | (reg_r << LD_R_OPCODE_SHIFT)) as usize
}

pub fn set_r_register_from_opcode(
    opcode: u8,
    value: u8,
    cpu: &mut Cpu,
) -> Result<Reg, InstructionError> {
    r_reg_from_opcode(opcode).map(|reg| {
        match reg {
            Reg::A => cpu.registers.set_a(value),
            Reg::B => cpu.registers.set_b(value),
            Reg::C => cpu.registers.set_c(value),
            Reg::D => cpu.registers.set_d(value),
            Reg::E => cpu.registers.set_e(value),
            Reg::H => cpu.registers.set_h(value),
            Reg::L => cpu.registers.set_l(value),
        };
        reg
    })
}

pub fn r_prime_reg_from_opcode(opcode: u8) -> u8 {
    opcode & R_REG_OPCODE_MASK
}

fn r_reg_from_opcode(opcode: u8) -> Result<Reg, InstructionError> {
    ((opcode >> LD_R_OPCODE_SHIFT) & R_REG_OPCODE_MASK).try_into()
}
