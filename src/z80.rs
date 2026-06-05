use crate::bus::{Bus, BusError};
use crate::z80::instructions::{INSTRUCTIONS, Instruction};
use alu::Alu;
use cpu_registers::CpuRegisters;

mod alu;
mod cpu_registers;
mod instructions;

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
    pub fn cycle(&mut self, bus: &Bus) -> Result<CycleResult, CycleError> {
        // Read opcode
        let opcode = bus
            .read(self.registers.pc)
            .map_err(|bus_error: BusError| CycleError::BusReadError(bus_error))?;

        // Increment PC
        self.registers.increment_pc();

        // Try decoding instruction
        let instruction = self.decode(opcode)?;
        (instruction.execute)(self, bus)?;

        Ok(CycleResult::Success)
    }

    pub fn dump(&self) {
        self.registers.dump();
    }

    // Handles decoding and finding a relevant instruction in the instruction table (todo() pages)
    fn decode(&self, opcode: u8) -> Result<&Instruction, CycleError> {
        let instruction = &INSTRUCTIONS[opcode as usize];
        Ok(instruction)
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: CpuRegisters::init(),
            alu: Alu,
        }
    }
}
