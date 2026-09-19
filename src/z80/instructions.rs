mod constants;
mod ld_r_hl;
mod ld_r_ix_d;
mod ld_r_n;
mod ld_r_r;
mod utils;

use crate::bus::{Bus, Readable};
use crate::z80::instructions::InstructionError::UnsupportedOpcode;
use crate::z80::instructions::ld_r_hl::build_ld_r_hl_set;
use crate::z80::instructions::ld_r_ix_d::build_ld_r_ix_d_set;
use crate::z80::instructions::ld_r_n::build_ld_r_n_set;
use crate::z80::instructions::ld_r_r::build_ld_r_r_set;
use crate::z80::utils::ReadableFromPc;
use crate::z80::{Cpu, CycleError};
use log::debug;

#[derive(Debug)]
pub enum InstructionError {
    UnsupportedOpcode,
    UnsupportedRegister,
}

#[derive(Copy, Clone)]
pub struct Cycles {
    m_cycles: u8,
    t_states: u8,
}

pub struct Instruction<B>
where
    B: Readable,
{
    cycles: Cycles,
    execute: for<'a> fn(context: &'a Context, &mut Cpu, &B) -> Result<&'a Context, CycleError>,
}

pub struct Context {
    opcode: u8,
}

pub struct InstructionData {
    instruction: &'static Instruction<Bus>,
    context: Context,
}

impl InstructionData {
    pub fn run_with_context(&self, cpu: &mut Cpu, bus: &Bus) -> Result<Cycles, CycleError> {
        self.instruction.run(&self.context, cpu, bus)
    }
}

impl Instruction<Bus> {
    pub fn decode(opcode: u8, cpu: &mut Cpu, bus: &Bus) -> Result<InstructionData, CycleError> {
        let (lookup_map, opcode) = if opcode == DD_OPCODE {
            let dd_opcode = bus.read_from_pc(cpu)?;
            (&INSTRUCTIONS_DD, dd_opcode)
        } else {
            (&INSTRUCTIONS, opcode)
        };
        Ok(InstructionData {
            instruction: &lookup_map[usize::from(opcode)],
            context: Context { opcode },
        })
    }

    fn run(&self, context: &Context, cpu: &mut Cpu, bus: &Bus) -> Result<Cycles, CycleError> {
        (self.execute)(context, cpu, bus)?;
        Ok(self.cycles)
    }
}

const NOP: u8 = 0x00;

const UNSUPPORTED_INSTRUCTION: Instruction<Bus> = Instruction {
    cycles: Cycles {
        m_cycles: 0,
        t_states: 0,
    },
    execute: |_, _, _| Err(CycleError::UnsupportedInstruction(UnsupportedOpcode)),
};

const fn build_instruction_map() -> [Instruction<Bus>; 256] {
    let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];

    build_ld_r_r_set(&mut instructions);
    build_ld_r_n_set(&mut instructions);
    build_ld_r_hl_set(&mut instructions);

    instructions[NOP as usize] = Instruction {
        cycles: Cycles {
            m_cycles: 1,
            t_states: 4,
        },
        execute: |context, _, _| {
            debug!("NOP");
            Ok(context)
        },
    };
    instructions
}

const fn build_dd_instruction_map() -> [Instruction<Bus>; 256] {
    let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];

    build_ld_r_ix_d_set(&mut instructions);

    instructions
}

const INSTRUCTIONS: [Instruction<Bus>; 256] = build_instruction_map();
const INSTRUCTIONS_DD: [Instruction<Bus>; 256] = build_dd_instruction_map();

const DD_OPCODE: u8 = 0xDD;
