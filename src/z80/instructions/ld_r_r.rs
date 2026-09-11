use crate::utils::ToHexString;
use crate::z80::CycleError;
use crate::z80::instructions::constants::{REGS, Reg, reg_code_from};
use crate::z80::instructions::utils::{r_prime_reg_from_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Cycles, Instruction};
use log::debug;
use crate::bus::Bus;

// LR r, r'
const fn opcode(reg: Reg, reg_prime: Reg) -> usize {
    const LD_R_R_OPCODE_MASK: u8 = 0x40;
    let reg_code: u8 = reg_code_from(reg);
    let reg_prime_code: u8 = reg_code_from(reg_prime);
    (LD_R_R_OPCODE_MASK | (reg_code << 3) | reg_prime_code) as usize
}

pub const fn build_ld_r_r_set(instructions: &mut [Instruction<Bus>]) {
    const LD_R_R_CYCLES: Cycles = Cycles {
        m_cycles: 1,
        t_states: 4,
    };

    let mut i = 0;
    while i < REGS.len() {
        let mut j = 0;
        while j < REGS.len() {
            instructions[opcode(REGS[i], REGS[j])] = Instruction {
                cycles: LD_R_R_CYCLES,
                execute: |context, cpu, _| {
                    let reg_prime = Reg::try_from(r_prime_reg_from_opcode(context.opcode))
                        .map_err(CycleError::UnsupportedInstruction)?;
                    let value = match reg_prime {
                        Reg::A => cpu.registers.get_a(),
                        Reg::B => cpu.registers.get_b(),
                        Reg::C => cpu.registers.get_c(),
                        Reg::D => cpu.registers.get_d(),
                        Reg::E => cpu.registers.get_e(),
                        Reg::H => cpu.registers.get_h(),
                        Reg::L => cpu.registers.get_l(),
                    };
                    let reg = set_r_register_from_opcode(context.opcode, value, cpu)
                        .map_err(CycleError::UnsupportedInstruction)?;
                    debug!(
                        "{} LD {:?}, {:?}",
                        context.opcode.to_hex_string(),
                        reg,
                        reg_prime
                    );
                    Ok(context)
                },
            };
            j += 1;
        }
        i += 1;
    }
}
