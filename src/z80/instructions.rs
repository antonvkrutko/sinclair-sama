pub mod ld_r_hl;
mod ld_r_n;
mod ld_r_r;

use crate::bus::Bus;
use crate::z80::instructions::ld_r_hl::build_ld_r_hl_set;
use crate::z80::instructions::ld_r_n::build_ld_r_n_set;
use crate::z80::instructions::ld_r_r::build_ld_r_r_set;
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
    build_ld_r_hl_set(&mut instructions);

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

pub const INSTRUCTIONS: [Instruction; 256] = build_instruction_map();
