use crate::utils::ToHexString;
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r, (HL)
const LD_A_HL: u8 = 0x7E;
const LD_B_HL: u8 = 0x46;
const LD_C_HL: u8 = 0x4E;
const LD_D_HL: u8 = 0x56;
const LD_E_HL: u8 = 0x5E;
const LD_H_HL: u8 = 0x66;
const LD_L_HL: u8 = 0x6E;

pub const fn build_ld_r_hl_set(instructions: &mut [Instruction]) {
    const LD_R_HL_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    instructions[LD_A_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD A,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_a(integer);
            Ok(())
        },
    };
    instructions[LD_B_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD B,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_b(integer);
            Ok(())
        },
    };
    instructions[LD_C_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD C,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_c(integer);
            Ok(())
        },
    };
    instructions[LD_D_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD D,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_d(integer);
            Ok(())
        },
    };
    instructions[LD_E_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD E,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_e(integer);
            Ok(())
        },
    };
    instructions[LD_H_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD H,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_h(integer);
            Ok(())
        },
    };
    instructions[LD_L_HL as usize] = Instruction {
        cycles: LD_R_HL_CYCLES,
        execute: |cpu, bus, logger| {
            let addr = cpu.registers.get_hl();
            let integer = bus.read_in_cycle(addr)?;

            logger(
                "LD L,",
                &[
                    &format!("({}) ->", addr.to_hex_string()),
                    &integer.to_hex_string(),
                ],
            );
            cpu.registers.set_l(integer);
            Ok(())
        },
    };
}
