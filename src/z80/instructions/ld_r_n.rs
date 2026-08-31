use crate::utils::ToHexString;
use crate::z80::CycleError;
use crate::z80::instructions::constants::REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;
use log::debug;

// LD r,n
const LD_R_N_OPCODE_MASK: u8 = 0x06;

pub const fn build_ld_r_n_set(instructions: &mut [Instruction]) {
    const LD_R_N_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    let mut reg_idx = 0;
    while reg_idx < REGS.len() {
        instructions[r_reg_opcode(&REGS[reg_idx], LD_R_N_OPCODE_MASK)] = Instruction {
            cycles: LD_R_N_CYCLES,
            execute: |context, cpu, bus| {
                let integer = bus.read_from_pc(cpu)?;
                let reg = set_r_register_from_opcode(context.opcode, integer, cpu)
                    .map_err(CycleError::UnsupportedInstruction)?;
                debug!(
                    "{} LD {:?}, {:}",
                    context.opcode.to_hex_string(),
                    reg,
                    integer.to_hex_string()
                );
                Ok(context)
            },
        };
        reg_idx += 1;
    }
}
