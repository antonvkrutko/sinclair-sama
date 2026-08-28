use crate::utils::ToHexString;
use crate::z80::instructions::constants::R_REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r, (HL)
const LD_R_HL_OPCODE_MASK: u8 = 0x46;

pub const fn build_ld_r_hl_set(instructions: &mut [Instruction]) {
    const LD_R_HL_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    let mut reg_idx = 0;
    while reg_idx < R_REGS.len() {
        let reg = R_REGS[reg_idx];
        instructions[r_reg_opcode(reg, LD_R_HL_OPCODE_MASK)] = Instruction {
            cycles: LD_R_HL_CYCLES,
            execute: |context, cpu, bus, logger| {
                let addr = cpu.registers.get_hl();
                let integer = bus.read_in_cycle(addr)?;
                let result =
                    set_r_register_from_opcode(context.opcode, integer, cpu);
                logger(
                    "LD A,",
                    &[
                        &format!("({}) ->", addr.to_hex_string()),
                        &integer.to_hex_string(),
                    ],
                );
                result
            },
        };
        reg_idx += 1;
    }
}
