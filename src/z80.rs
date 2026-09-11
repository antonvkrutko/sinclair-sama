use crate::bus::{Bus, BusError, Readable};
use crate::z80::instructions::{Instruction, InstructionData, InstructionError};
use cpu_registers::CpuRegisters;

mod cpu_registers;
mod instructions;
mod utils;

pub struct Cpu {
    registers: CpuRegisters,
}

pub enum CycleResult {
    Success,
}

#[derive(Debug)]
pub enum CycleError {
    BusReadError(BusError),
    UnsupportedInstruction(InstructionError),
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: CpuRegisters::new(),
        }
    }

    pub fn cycle(&mut self, bus: &Bus) -> Result<CycleResult, CycleError> {
        // Try decoding instruction
        let instruction_data = self.fetch_instruction(bus)?;
        instruction_data.run_with_context(self, bus)?;

        Ok(CycleResult::Success)
    }

    pub fn dump(&self) {
        self.registers.dump();
    }

    fn fetch_instruction(&mut self, bus: &Bus) -> Result<InstructionData, CycleError> {
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
