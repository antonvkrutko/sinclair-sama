use crate::bus::{Bus, BusError};
use crate::z80::instructions::Instruction;
use alu::Alu;
use cpu_registers::CpuRegisters;

mod alu;
mod cpu_registers;
mod instructions;
mod utils;

pub struct Cpu {
    registers: CpuRegisters,
    alu: Alu,
}

pub enum CycleResult {
    Success,
}

#[derive(Debug)]
pub enum CycleError {
    BusReadError(BusError),
    UnsupportedInstruction,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: CpuRegisters::new(),
            alu: Alu,
        }
    }

    pub fn cycle(&mut self, bus: &Bus) -> Result<CycleResult, CycleError> {
        // Try decoding instruction
        let instruction = self.fetch_instruction(bus)?;
        instruction.run(self, bus)?;

        Ok(CycleResult::Success)
    }

    pub fn dump(&self) {
        self.registers.dump();
    }

    // Handles decoding and finding a relevant instruction in the instruction table (todo() pages)
    fn fetch_instruction(&mut self, bus: &Bus) -> Result<&'static Instruction, CycleError> {
        // Read opcode
        let opcode = bus
            .read(self.registers.current_pc())
            .map_err(CycleError::BusReadError)?;

        // Increment PC
        self.registers.increment_pc();

        // Decode instruction
        Instruction::decode(opcode, self, bus)
    }
}
