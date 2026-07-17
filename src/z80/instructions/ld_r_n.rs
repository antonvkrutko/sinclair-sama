use crate::utils::ToHexString;
use crate::z80::instructions::registers::{A_REG, B_REG, C_REG, D_REG, E_REG, H_REG, L_REG};
use crate::z80::instructions::{Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;

// LD r,n
const fn opcode(reg_r: u8) -> usize {
    const LR_R_N_OPCODE_MASK: u8 = 0x06;
    (LR_R_N_OPCODE_MASK | (reg_r << 3)) as usize
}

pub const fn build_ld_r_n_set(instructions: &mut [Instruction]) {
    const LD_R_N_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    instructions[opcode(A_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD A,", &[&integer.to_hex_string()]);
            cpu.registers.set_a(integer);
            Ok(())
        },
    };
    instructions[opcode(B_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD B,", &[&integer.to_hex_string()]);
            cpu.registers.set_b(integer);
            Ok(())
        },
    };
    instructions[opcode(C_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD C,", &[&integer.to_hex_string()]);
            cpu.registers.set_c(integer);
            Ok(())
        },
    };
    instructions[opcode(D_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD D,", &[&integer.to_hex_string()]);
            cpu.registers.set_d(integer);
            Ok(())
        },
    };
    instructions[opcode(E_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD E,", &[&integer.to_hex_string()]);
            cpu.registers.set_e(integer);
            Ok(())
        },
    };
    instructions[opcode(H_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD H,", &[&integer.to_hex_string()]);
            cpu.registers.set_h(integer);
            Ok(())
        },
    };
    instructions[opcode(L_REG)] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD L,", &[&integer.to_hex_string()]);
            cpu.registers.set_l(integer);
            Ok(())
        },
    };
}

#[cfg(test)]
mod test {
    use crate::bus::Bus;
    use crate::z80::instructions::ld_r_n::{build_ld_r_n_set, opcode};
    use crate::z80::instructions::registers::A_REG;
    use crate::z80::instructions::UNSUPPORTED_INSTRUCTION;
    use crate::z80::Cpu;

    fn test_cpu() -> Cpu {
        Cpu::new()
    }

    fn test_bus() -> Bus {
        Bus::new()
    }

    fn noop_logger(_: &str, _: &[&str]) {}

    #[test]
    fn test_opcode() {
        let ld_a_n = opcode(A_REG);
        assert_eq!(ld_a_n, 0x3E)
    }

    #[test]
    fn test_ld_a_n() {
        let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];
        build_ld_r_n_set(&mut instructions);
        let ld_a_n = &instructions[opcode(A_REG)].execute;

        let mut bus = test_bus();
        let rom = [42u8];
        bus.load_rom_raw(&rom);

        let mut cpu = test_cpu();
        ld_a_n(&mut cpu, &bus, &noop_logger).unwrap();

        assert_eq!(cpu.registers.get_a(), 42)
    }
}
