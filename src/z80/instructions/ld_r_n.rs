use crate::utils::ToHexString;
use crate::z80::instructions::constants::R_REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r,n
const LR_R_N_OPCODE_MASK: u8 = 0x06;

pub const fn build_ld_r_n_set(instructions: &mut [Instruction]) {
    const LD_R_N_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    let mut reg_idx = 0;
    while reg_idx < R_REGS.len() {
        let reg = R_REGS[reg_idx];
        instructions[r_reg_opcode(reg, LR_R_N_OPCODE_MASK)] = Instruction {
            cycles: LD_R_N_CYCLES,
            execute: |context, cpu, bus, logger| {
                let integer = bus.read_from_pc(cpu)?;
                let result =
                    set_r_register_from_opcode(context.opcode, integer, cpu);
                logger("LD r,", &[&integer.to_hex_string()]);
                result
            },
        };
        reg_idx += 1;
    }
}
