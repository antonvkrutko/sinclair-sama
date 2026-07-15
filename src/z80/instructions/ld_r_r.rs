use crate::z80::instructions::registers::{A_REG, B_REG, C_REG, D_REG, E_REG, H_REG, L_REG};
use crate::z80::instructions::{Cycles, Instruction};
const fn opcode(reg_r: u8, reg_r_: u8) -> usize {
    const LD_R_R_OPCODE_PREFIX: u8 = 0x01;
    ((LD_R_R_OPCODE_PREFIX << 6) | (reg_r << 3) | reg_r_) as usize
}

// LR r, r'
pub const fn build_ld_r_r_set(instructions: &mut [Instruction]) {
    const LD_R_R_CYCLES: Cycles = Cycles {
        m_cycles: 1,
        t_states: 4,
    };

    instructions[opcode(A_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, A", &[]);
            cpu.registers.set_a(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(A_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, B", &[]);
            cpu.registers.set_a(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(A_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, C", &[]);
            cpu.registers.set_a(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(A_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, D", &[]);
            cpu.registers.set_a(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(A_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, E", &[]);
            cpu.registers.set_a(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(A_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, H", &[]);
            cpu.registers.set_a(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(A_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, L", &[]);
            cpu.registers.set_a(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(B_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, A", &[]);
            cpu.registers.set_b(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(B_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, B", &[]);
            cpu.registers.set_b(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(B_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, C", &[]);
            cpu.registers.set_b(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(B_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, D", &[]);
            cpu.registers.set_b(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(B_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, E", &[]);
            cpu.registers.set_b(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(B_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, H", &[]);
            cpu.registers.set_b(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(B_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, L", &[]);
            cpu.registers.set_b(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(C_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, A", &[]);
            cpu.registers.set_c(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(C_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, B", &[]);
            cpu.registers.set_c(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(C_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, C", &[]);
            cpu.registers.set_c(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(C_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, D", &[]);
            cpu.registers.set_c(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(C_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, E", &[]);
            cpu.registers.set_c(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(C_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, H", &[]);
            cpu.registers.set_c(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(C_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, L", &[]);
            cpu.registers.set_c(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(D_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, A", &[]);
            cpu.registers.set_d(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(D_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, B", &[]);
            cpu.registers.set_d(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(D_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, C", &[]);
            cpu.registers.set_d(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(D_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, D", &[]);
            cpu.registers.set_d(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(D_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, E", &[]);
            cpu.registers.set_d(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(D_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, H", &[]);
            cpu.registers.set_d(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(D_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, L", &[]);
            cpu.registers.set_d(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(E_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, A", &[]);
            cpu.registers.set_e(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(E_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, B", &[]);
            cpu.registers.set_e(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(E_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, C", &[]);
            cpu.registers.set_e(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(E_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, D", &[]);
            cpu.registers.set_e(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(E_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, E", &[]);
            cpu.registers.set_e(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(E_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, H", &[]);
            cpu.registers.set_e(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(E_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, L", &[]);
            cpu.registers.set_e(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(H_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, A", &[]);
            cpu.registers.set_h(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(H_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, B", &[]);
            cpu.registers.set_h(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(H_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, C", &[]);
            cpu.registers.set_h(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(H_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, D", &[]);
            cpu.registers.set_h(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(H_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, E", &[]);
            cpu.registers.set_h(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(H_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, H", &[]);
            cpu.registers.set_h(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(H_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, L", &[]);
            cpu.registers.set_h(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[opcode(L_REG, A_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, A", &[]);
            cpu.registers.set_l(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[opcode(L_REG, B_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, B", &[]);
            cpu.registers.set_l(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[opcode(L_REG, C_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, C", &[]);
            cpu.registers.set_l(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[opcode(L_REG, D_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, D", &[]);
            cpu.registers.set_l(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[opcode(L_REG, E_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, E", &[]);
            cpu.registers.set_l(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[opcode(L_REG, H_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, H", &[]);
            cpu.registers.set_l(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[opcode(L_REG, L_REG)] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, L", &[]);
            cpu.registers.set_l(cpu.registers.get_l());
            Ok(())
        },
    };
}

#[cfg(test)]
mod test {
    use crate::bus::Bus;
    use crate::z80::instructions::ld_r_r::{build_ld_r_r_set, opcode};
    use crate::z80::instructions::registers::{A_REG, B_REG, C_REG, D_REG, E_REG, H_REG, L_REG};
    use crate::z80::instructions::UNSUPPORTED_INSTRUCTION;
    use crate::z80::Cpu;

    #[test]
    fn test_opcode() {
        const A: u8 = 0b111;
        const B: u8 = 0b000;
        assert_eq!(opcode(A, B), 0x78);
    }

    fn create_test_cpu() -> Cpu {
        Cpu::new()
    }

    fn noop_logger(_: &str, _: &[&str]) {}

    fn execute_instruction(cpu: &mut Cpu, opcode: usize) {
        let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];
        build_ld_r_r_set(&mut instructions);
        let bus = Bus::new();
        (instructions[opcode].execute)(cpu, &bus, &noop_logger).unwrap();
    }

    #[test]
    fn test_ld_a_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x42);
        execute_instruction(&mut cpu, opcode(A_REG, B_REG));
        assert_eq!(cpu.registers.get_a(), 0x42);
    }

    #[test]
    fn test_ld_a_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x43);
        execute_instruction(&mut cpu, opcode(A_REG, C_REG));
        assert_eq!(cpu.registers.get_a(), 0x43);
    }

    #[test]
    fn test_ld_a_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x44);
        execute_instruction(&mut cpu, opcode(A_REG, D_REG));
        assert_eq!(cpu.registers.get_a(), 0x44);
    }

    #[test]
    fn test_ld_a_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x45);
        execute_instruction(&mut cpu, opcode(A_REG, E_REG));
        assert_eq!(cpu.registers.get_a(), 0x45);
    }

    #[test]
    fn test_ld_a_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x46);
        execute_instruction(&mut cpu, opcode(A_REG, H_REG));
        assert_eq!(cpu.registers.get_a(), 0x46);
    }

    #[test]
    fn test_ld_a_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x47);
        execute_instruction(&mut cpu, opcode(A_REG, L_REG));
        assert_eq!(cpu.registers.get_a(), 0x47);
    }

    #[test]
    fn test_ld_b_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x48);
        execute_instruction(&mut cpu, opcode(B_REG, A_REG));
        assert_eq!(cpu.registers.get_b(), 0x48);
    }

    #[test]
    fn test_ld_b_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x49);
        execute_instruction(&mut cpu, opcode(B_REG, C_REG));
        assert_eq!(cpu.registers.get_b(), 0x49);
    }

    #[test]
    fn test_ld_b_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x4A);
        execute_instruction(&mut cpu, opcode(B_REG, D_REG));
        assert_eq!(cpu.registers.get_b(), 0x4A);
    }

    #[test]
    fn test_ld_b_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x4B);
        execute_instruction(&mut cpu, opcode(B_REG, E_REG));
        assert_eq!(cpu.registers.get_b(), 0x4B);
    }

    #[test]
    fn test_ld_b_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x4C);
        execute_instruction(&mut cpu, opcode(B_REG, H_REG));
        assert_eq!(cpu.registers.get_b(), 0x4C);
    }

    #[test]
    fn test_ld_b_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x4D);
        execute_instruction(&mut cpu, opcode(B_REG, L_REG));
        assert_eq!(cpu.registers.get_b(), 0x4D);
    }

    #[test]
    fn test_ld_c_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x4E);
        execute_instruction(&mut cpu, opcode(C_REG, A_REG));
        assert_eq!(cpu.registers.get_c(), 0x4E);
    }

    #[test]
    fn test_ld_c_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x4F);
        execute_instruction(&mut cpu, opcode(C_REG, B_REG));
        assert_eq!(cpu.registers.get_c(), 0x4F);
    }

    #[test]
    fn test_ld_c_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x50);
        execute_instruction(&mut cpu, opcode(C_REG, D_REG));
        assert_eq!(cpu.registers.get_c(), 0x50);
    }

    #[test]
    fn test_ld_c_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x51);
        execute_instruction(&mut cpu, opcode(C_REG, E_REG));
        assert_eq!(cpu.registers.get_c(), 0x51);
    }

    #[test]
    fn test_ld_c_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x52);
        execute_instruction(&mut cpu, opcode(C_REG, H_REG));
        assert_eq!(cpu.registers.get_c(), 0x52);
    }

    #[test]
    fn test_ld_c_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x53);
        execute_instruction(&mut cpu, opcode(C_REG, L_REG));
        assert_eq!(cpu.registers.get_c(), 0x53);
    }

    #[test]
    fn test_ld_d_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x54);
        execute_instruction(&mut cpu, opcode(D_REG, A_REG));
        assert_eq!(cpu.registers.get_d(), 0x54);
    }

    #[test]
    fn test_ld_d_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x55);
        execute_instruction(&mut cpu, opcode(D_REG, B_REG));
        assert_eq!(cpu.registers.get_d(), 0x55);
    }

    #[test]
    fn test_ld_d_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x56);
        execute_instruction(&mut cpu, opcode(D_REG, C_REG));
        assert_eq!(cpu.registers.get_d(), 0x56);
    }

    #[test]
    fn test_ld_d_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x57);
        execute_instruction(&mut cpu, opcode(D_REG, E_REG));
        assert_eq!(cpu.registers.get_d(), 0x57);
    }

    #[test]
    fn test_ld_d_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x58);
        execute_instruction(&mut cpu, opcode(D_REG, H_REG));
        assert_eq!(cpu.registers.get_d(), 0x58);
    }

    #[test]
    fn test_ld_d_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x59);
        execute_instruction(&mut cpu, opcode(D_REG, L_REG));
        assert_eq!(cpu.registers.get_d(), 0x59);
    }

    #[test]
    fn test_ld_e_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x5A);
        execute_instruction(&mut cpu, opcode(E_REG, A_REG));
        assert_eq!(cpu.registers.get_e(), 0x5A);
    }

    #[test]
    fn test_ld_e_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x5B);
        execute_instruction(&mut cpu, opcode(E_REG, B_REG));
        assert_eq!(cpu.registers.get_e(), 0x5B);
    }

    #[test]
    fn test_ld_e_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x5C);
        execute_instruction(&mut cpu, opcode(E_REG, C_REG));
        assert_eq!(cpu.registers.get_e(), 0x5C);
    }

    #[test]
    fn test_ld_e_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x5D);
        execute_instruction(&mut cpu, opcode(E_REG, D_REG));
        assert_eq!(cpu.registers.get_e(), 0x5D);
    }

    #[test]
    fn test_ld_e_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x5E);
        execute_instruction(&mut cpu, opcode(E_REG, H_REG));
        assert_eq!(cpu.registers.get_e(), 0x5E);
    }

    #[test]
    fn test_ld_e_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x5F);
        execute_instruction(&mut cpu, opcode(E_REG, L_REG));
        assert_eq!(cpu.registers.get_e(), 0x5F);
    }

    #[test]
    fn test_ld_h_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x60);
        execute_instruction(&mut cpu, opcode(H_REG, A_REG));
        assert_eq!(cpu.registers.get_h(), 0x60);
    }

    #[test]
    fn test_ld_h_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x61);
        execute_instruction(&mut cpu, opcode(H_REG, B_REG));
        assert_eq!(cpu.registers.get_h(), 0x61);
    }

    #[test]
    fn test_ld_h_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x62);
        execute_instruction(&mut cpu, opcode(H_REG, C_REG));
        assert_eq!(cpu.registers.get_h(), 0x62);
    }

    #[test]
    fn test_ld_h_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x63);
        execute_instruction(&mut cpu, opcode(H_REG, D_REG));
        assert_eq!(cpu.registers.get_h(), 0x63);
    }

    #[test]
    fn test_ld_h_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x64);
        execute_instruction(&mut cpu, opcode(H_REG, E_REG));
        assert_eq!(cpu.registers.get_h(), 0x64);
    }

    #[test]
    fn test_ld_h_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x65);
        execute_instruction(&mut cpu, opcode(H_REG, L_REG));
        assert_eq!(cpu.registers.get_h(), 0x65);
    }

    #[test]
    fn test_ld_l_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x66);
        execute_instruction(&mut cpu, opcode(L_REG, A_REG));
        assert_eq!(cpu.registers.get_l(), 0x66);
    }

    #[test]
    fn test_ld_l_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x67);
        execute_instruction(&mut cpu, opcode(L_REG, B_REG));
        assert_eq!(cpu.registers.get_l(), 0x67);
    }

    #[test]
    fn test_ld_l_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x68);
        execute_instruction(&mut cpu, opcode(L_REG, C_REG));
        assert_eq!(cpu.registers.get_l(), 0x68);
    }

    #[test]
    fn test_ld_l_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x69);
        execute_instruction(&mut cpu, opcode(L_REG, D_REG));
        assert_eq!(cpu.registers.get_l(), 0x69);
    }

    #[test]
    fn test_ld_l_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x6A);
        execute_instruction(&mut cpu, opcode(L_REG, E_REG));
        assert_eq!(cpu.registers.get_l(), 0x6A);
    }

    #[test]
    fn test_ld_l_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x6B);
        execute_instruction(&mut cpu, opcode(L_REG, H_REG));
        assert_eq!(cpu.registers.get_l(), 0x6B);
    }

    #[test]
    fn test_ld_a_a() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_a(0x6C);
        execute_instruction(&mut cpu, opcode(A_REG, A_REG));
        assert_eq!(cpu.registers.get_a(), 0x6C);
    }

    #[test]
    fn test_ld_b_b() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_b(0x6D);
        execute_instruction(&mut cpu, opcode(B_REG, B_REG));
        assert_eq!(cpu.registers.get_b(), 0x6D);
    }

    #[test]
    fn test_ld_c_c() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_c(0x6E);
        execute_instruction(&mut cpu, opcode(C_REG, C_REG));
        assert_eq!(cpu.registers.get_c(), 0x6E);
    }

    #[test]
    fn test_ld_d_d() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_d(0x6F);
        execute_instruction(&mut cpu, opcode(D_REG, D_REG));
        assert_eq!(cpu.registers.get_d(), 0x6F);
    }

    #[test]
    fn test_ld_e_e() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_e(0x70);
        execute_instruction(&mut cpu, opcode(E_REG, E_REG));
        assert_eq!(cpu.registers.get_e(), 0x70);
    }

    #[test]
    fn test_ld_h_h() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_h(0x71);
        execute_instruction(&mut cpu, opcode(H_REG, H_REG));
        assert_eq!(cpu.registers.get_h(), 0x71);
    }

    #[test]
    fn test_ld_l_l() {
        let mut cpu = create_test_cpu();
        cpu.registers.set_l(0x72);
        execute_instruction(&mut cpu, opcode(L_REG, L_REG));
        assert_eq!(cpu.registers.get_l(), 0x72);
    }
}
