use crate::bus::Bus;
use crate::z80::{Cpu, CycleError};

pub struct Instruction {
    pub execute: fn(&mut Cpu, &Bus) -> Result<(), CycleError>,
}

const NOP: u8 = 0x00;
const LD_A_N: u8 = 0x3E;

const UNSUPPORTED_INSTRUCTION: Instruction = Instruction {
    execute: |_, _| Err(CycleError::UnsupportedInstruction),
};
const fn build_instruction_map() -> [Instruction; 256] {
    let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];

    instructions[NOP as usize] = Instruction {
        execute: |_, _| {
            println!("NOP");
            Ok(())
        },
    };
    instructions[LD_A_N as usize] = Instruction {
        execute: |cpu: &mut Cpu, bus: &Bus| {
            let integer = bus
                .read(cpu.registers.pc)
                .map_err(CycleError::BusReadError)?;

            println!("LD A, 0x{:X}", integer);

            cpu.registers.increment_pc();
            cpu.registers.set_a(integer);
            Ok(())
        },
    };

    instructions
}

pub const INSTRUCTIONS: [Instruction; 256] = build_instruction_map();
