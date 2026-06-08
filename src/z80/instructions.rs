use crate::bus::Bus;
use crate::z80::{Cpu, CycleError};

pub struct Instruction {
    pub execute: fn(&mut Cpu, &Bus) -> Result<(), CycleError>,
}

const NOP: u8 = 0x00;
const LD_A_N: u8 = 0x3E;
const LD_B_N: u8 = 0x06;
const LD_C_N: u8 = 0x0E;
const LD_D_N: u8 = 0x16;
const LD_E_N: u8 = 0x1E;
const LD_H_N: u8 = 0x26;
const LD_L_N: u8 = 0x2E;

const UNSUPPORTED_INSTRUCTION: Instruction = Instruction {
    execute: |_, _| Err(CycleError::UnsupportedInstruction),
};
const fn build_instruction_map() -> [Instruction; 256] {
    let mut instructions = [UNSUPPORTED_INSTRUCTION; 256];

    instructions[NOP as usize] = Instruction {
        execute: |_, _| {
            println!("-- Executing: NOP");
            Ok(())
        },
    };
    instructions[LD_A_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD A, 0x{:X}", integer);
            cpu.registers.set_a(integer);
            Ok(())
        },
    };
    instructions[LD_B_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD B, 0x{:X}", integer);
            cpu.registers.set_b(integer);
            Ok(())
        },
    };
    instructions[LD_C_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD C, 0x{:X}", integer);
            cpu.registers.set_c(integer);
            Ok(())
        },
    };
    instructions[LD_D_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD D, 0x{:X}", integer);
            cpu.registers.set_d(integer);
            Ok(())
        },
    };
    instructions[LD_E_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD E, 0x{:X}", integer);
            cpu.registers.set_e(integer);
            Ok(())
        },
    };
    instructions[LD_H_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD H, 0x{:X}", integer);
            cpu.registers.set_h(integer);
            Ok(())
        },
    };
    instructions[LD_L_N as usize] = Instruction {
        execute: |cpu, bus| {
            let integer = bus.read_from_pc(cpu)?;

            println!("-- Executing: LD L, 0x{:X}", integer);
            cpu.registers.set_l(integer);
            Ok(())
        },
    };
    instructions
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
