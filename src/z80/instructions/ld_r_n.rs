use crate::utils::ToHexString;
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r,n
const LD_A_N: u8 = 0x3E;
const LD_B_N: u8 = 0x06;
const LD_C_N: u8 = 0x0E;
const LD_D_N: u8 = 0x16;
const LD_E_N: u8 = 0x1E;
const LD_H_N: u8 = 0x26;
const LD_L_N: u8 = 0x2E;

pub const fn build_ld_r_n_set(instructions: &mut [Instruction]) {
    const LD_R_N_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    instructions[LD_A_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD A,", &[&integer.to_hex_string()]);
            cpu.registers.set_a(integer);
            Ok(())
        },
    };
    instructions[LD_B_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD B,", &[&integer.to_hex_string()]);
            cpu.registers.set_b(integer);
            Ok(())
        },
    };
    instructions[LD_C_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD C,", &[&integer.to_hex_string()]);
            cpu.registers.set_c(integer);
            Ok(())
        },
    };
    instructions[LD_D_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD D,", &[&integer.to_hex_string()]);
            cpu.registers.set_d(integer);
            Ok(())
        },
    };
    instructions[LD_E_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD E,", &[&integer.to_hex_string()]);
            cpu.registers.set_e(integer);
            Ok(())
        },
    };
    instructions[LD_H_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD H,", &[&integer.to_hex_string()]);
            cpu.registers.set_h(integer);
            Ok(())
        },
    };
    instructions[LD_L_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD L,", &[&integer.to_hex_string()]);
            cpu.registers.set_l(integer);
            Ok(())
        },
    };
}
