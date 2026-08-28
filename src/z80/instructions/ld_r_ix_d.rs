use crate::z80::instructions::constants::R_REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r,(IX + d)
const LR_R_IX_D_OPCODE_MASK: u8 = 0x46;

pub const fn build_ld_r_ix_d_set(instructions: &mut [Instruction]) {
    const LD_R_IX_D_CYCLES: Cycles = Cycles {
        m_cycles: 5,
        t_states: 19,
    };

    let mut reg_idx = 0;
    while reg_idx < R_REGS.len() {
        instructions[r_reg_opcode(R_REGS[reg_idx], LR_R_IX_D_OPCODE_MASK)] = Instruction {
            cycles: LD_R_IX_D_CYCLES,
            execute: |context, cpu, bus, logger| {
                let displacement = bus.read_from_pc(cpu)? as i8;
                let ix = cpu.registers.get_ix();
                let addr = ix.wrapping_add(displacement as u16);

                let value = bus.read_in_cycle(addr)?;
                set_r_register_from_opcode(context.opcode, value, cpu)
            },
        };
        reg_idx += 1;
    }
}
