use crate::bus::Bus;
use crate::z80::{Cpu, CycleError};

type Logger = dyn Fn(&str, &[&str]);

#[derive(Copy, Clone)]
pub struct Cycles {
    m_cycles: u8,
    t_states: u8,
}

pub struct Instruction {
    cycles: Cycles,
    execute: fn(&mut Cpu, &Bus, &Logger) -> Result<(), CycleError>,
}

impl Instruction {
    pub fn run(&self, cpu: &mut Cpu, bus: &Bus) -> Result<Cycles, CycleError> {
        let logger = |mnemonic: &str, args: &[&str]| {
            println!("{} {}", mnemonic, args.join(" "));
        };

        (self.execute)(cpu, bus, &logger)?;
        Ok(self.cycles)
    }
}

const NOP: u8 = 0x00;

// LR r, r'
const LD_A_B: u8 = 0x78;
const LD_A_C: u8 = 0x79;
const LD_A_D: u8 = 0x7A;
const LD_A_E: u8 = 0x7B;
const LD_A_H: u8 = 0x7C;
const LD_A_L: u8 = 0x7D;

const LD_B_A: u8 = 0x47;
const LD_B_C: u8 = 0x41;
const LD_B_D: u8 = 0x42;
const LD_B_E: u8 = 0x43;
const LD_B_H: u8 = 0x44;
const LD_B_L: u8 = 0x45;

const LD_C_A: u8 = 0x4F;
const LD_C_B: u8 = 0x49;
const LD_C_D: u8 = 0x4A;
const LD_C_E: u8 = 0x4B;
const LD_C_H: u8 = 0x4C;
const LD_C_L: u8 = 0x4D;

const LD_D_A: u8 = 0x57;
const LD_D_B: u8 = 0x50;
const LD_D_C: u8 = 0x51;
const LD_D_E: u8 = 0x53;
const LD_D_H: u8 = 0x54;
const LD_D_L: u8 = 0x55;

const LD_E_A: u8 = 0x5F;
const LD_E_B: u8 = 0x58;
const LD_E_C: u8 = 0x59;
const LD_E_D: u8 = 0x5A;
const LD_E_H: u8 = 0x5C;
const LD_E_L: u8 = 0x5D;

const LD_H_A: u8 = 0x67;
const LD_H_B: u8 = 0x60;
const LD_H_C: u8 = 0x61;
const LD_H_D: u8 = 0x62;
const LD_H_E: u8 = 0x63;
const LD_H_L: u8 = 0x65;

const LD_L_A: u8 = 0x6F;
const LD_L_B: u8 = 0x68;
const LD_L_C: u8 = 0x69;
const LD_L_D: u8 = 0x6A;
const LD_L_E: u8 = 0x6B;
const LD_L_H: u8 = 0x6C;

// LD r,n
const LD_A_N: u8 = 0x3E;
const LD_B_N: u8 = 0x06;
const LD_C_N: u8 = 0x0E;
const LD_D_N: u8 = 0x16;
const LD_E_N: u8 = 0x1E;
const LD_H_N: u8 = 0x26;
const LD_L_N: u8 = 0x2E;

const UNSUPPORTED_INSTRUCTION: Instruction = Instruction {
    cycles: Cycles {
        m_cycles: 0,
        t_states: 0,
    },
    execute: |_, _, _| Err(CycleError::UnsupportedInstruction),
};

const fn build_instruction_map() -> [Instruction; 256] {
    let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];

    build_ld_r_r_set(&mut instructions);
    build_ld_r_n_set(&mut instructions);

    instructions[NOP as usize] = Instruction {
        cycles: Cycles {
            m_cycles: 1,
            t_states: 4,
        },
        execute: |_, _, logger| {
            logger("NOP", &[]);
            Ok(())
        },
    };
    instructions
}

const fn build_ld_r_r_set(instructions: &mut [Instruction]) {
    const LD_R_R_CYCLES: Cycles = Cycles {
        m_cycles: 1,
        t_states: 4,
    };

    instructions[LD_A_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, B", &[]);
            cpu.registers.set_a(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_A_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, C", &[]);
            cpu.registers.set_a(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_A_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, D", &[]);
            cpu.registers.set_a(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_A_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, E", &[]);
            cpu.registers.set_a(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_A_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, H", &[]);
            cpu.registers.set_a(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[LD_A_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD A, L", &[]);
            cpu.registers.set_a(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_B_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, A", &[]);
            cpu.registers.set_b(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_B_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, C", &[]);
            cpu.registers.set_b(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_B_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, D", &[]);
            cpu.registers.set_b(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_B_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, E", &[]);
            cpu.registers.set_b(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_B_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, H", &[]);
            cpu.registers.set_b(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[LD_B_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD B, L", &[]);
            cpu.registers.set_b(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_C_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, A", &[]);
            cpu.registers.set_c(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_C_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, B", &[]);
            cpu.registers.set_c(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_C_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, D", &[]);
            cpu.registers.set_c(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_C_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, E", &[]);
            cpu.registers.set_c(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_C_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, H", &[]);
            cpu.registers.set_c(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[LD_C_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD C, L", &[]);
            cpu.registers.set_c(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_D_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, A", &[]);
            cpu.registers.set_d(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_D_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, B", &[]);
            cpu.registers.set_d(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_D_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, C", &[]);
            cpu.registers.set_d(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_D_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, E", &[]);
            cpu.registers.set_d(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_D_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, H", &[]);
            cpu.registers.set_d(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[LD_D_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD D, L", &[]);
            cpu.registers.set_d(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_E_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, A", &[]);
            cpu.registers.set_e(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_E_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, B", &[]);
            cpu.registers.set_e(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_E_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, C", &[]);
            cpu.registers.set_e(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_E_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, D", &[]);
            cpu.registers.set_e(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_E_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, H", &[]);
            cpu.registers.set_e(cpu.registers.get_h());
            Ok(())
        },
    };
    instructions[LD_E_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD E, L", &[]);
            cpu.registers.set_e(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_H_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, A", &[]);
            cpu.registers.set_h(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_H_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, B", &[]);
            cpu.registers.set_h(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_H_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, C", &[]);
            cpu.registers.set_h(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_H_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, D", &[]);
            cpu.registers.set_h(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_H_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, E", &[]);
            cpu.registers.set_h(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_H_L as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD H, L", &[]);
            cpu.registers.set_h(cpu.registers.get_l());
            Ok(())
        },
    };

    instructions[LD_L_A as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, A", &[]);
            cpu.registers.set_l(cpu.registers.get_a());
            Ok(())
        },
    };
    instructions[LD_L_B as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, B", &[]);
            cpu.registers.set_l(cpu.registers.get_b());
            Ok(())
        },
    };
    instructions[LD_L_C as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, C", &[]);
            cpu.registers.set_l(cpu.registers.get_c());
            Ok(())
        },
    };
    instructions[LD_L_D as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, D", &[]);
            cpu.registers.set_l(cpu.registers.get_d());
            Ok(())
        },
    };
    instructions[LD_L_E as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, E", &[]);
            cpu.registers.set_l(cpu.registers.get_e());
            Ok(())
        },
    };
    instructions[LD_L_H as usize] = Instruction {
        cycles: LD_R_R_CYCLES,
        execute: |cpu, _, logger| {
            logger("LD L, H", &[]);
            cpu.registers.set_l(cpu.registers.get_h());
            Ok(())
        },
    };
}

const fn build_ld_r_n_set(instructions: &mut [Instruction]) {
    const LD_R_N_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    instructions[LD_A_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD A, ", &[&integer.to_hex_string()]);
            cpu.registers.set_a(integer);
            Ok(())
        },
    };
    instructions[LD_B_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD B, ", &[&integer.to_hex_string()]);
            cpu.registers.set_b(integer);
            Ok(())
        },
    };
    instructions[LD_C_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD C, ", &[&integer.to_hex_string()]);
            cpu.registers.set_c(integer);
            Ok(())
        },
    };
    instructions[LD_D_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD D, ", &[&integer.to_hex_string()]);
            cpu.registers.set_d(integer);
            Ok(())
        },
    };
    instructions[LD_E_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD E, ", &[&integer.to_hex_string()]);
            cpu.registers.set_e(integer);
            Ok(())
        },
    };
    instructions[LD_H_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD H, ", &[&integer.to_hex_string()]);
            cpu.registers.set_h(integer);
            Ok(())
        },
    };
    instructions[LD_L_N as usize] = Instruction {
        cycles: LD_R_N_CYCLES,
        execute: |cpu, bus, logger| {
            let integer = bus.read_from_pc(cpu)?;

            logger("LD L, ", &[&integer.to_hex_string()]);
            cpu.registers.set_l(integer);
            Ok(())
        },
    };
}

trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for u8 {
    fn to_hex_string(&self) -> String {
        format!("0x{:0X}", self)
    }
}

trait ReadableFromPc {
    fn read_from_pc(&self, cpu: &mut Cpu) -> Result<u8, CycleError>;
}

impl ReadableFromPc for Bus {
    fn read_from_pc(&self, cpu: &mut Cpu) -> Result<u8, CycleError> {
        let value = self
            .read(cpu.registers.current_pc())
            .map_err(CycleError::BusReadError)?;
        cpu.registers.increment_pc();
        Ok(value)
    }
}

pub const INSTRUCTIONS: [Instruction; 256] = build_instruction_map();
