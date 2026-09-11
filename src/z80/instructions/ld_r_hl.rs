use crate::bus::Readable;
use crate::utils::ToHexString;
use crate::z80::instructions::constants::REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Context, Cycles, Instruction};
use crate::z80::utils::ReadableFromPc;
use crate::z80::{Cpu, CycleError};
use log::debug;

// LD r, (HL)
const LD_R_HL_OPCODE_MASK: u8 = 0x46;

pub const fn build_ld_r_hl_set<B>(instructions: &mut [Instruction<B>])
where
    B: Readable + ReadableFromPc,
{
    const LD_R_HL_CYCLES: Cycles = Cycles {
        m_cycles: 2,
        t_states: 7,
    };

    let mut reg_idx = 0;
    while reg_idx < REGS.len() {
        instructions[r_reg_opcode(REGS[reg_idx], LD_R_HL_OPCODE_MASK)] = Instruction {
            cycles: LD_R_HL_CYCLES,
            execute: execute_ld_r_hl,
        };
        reg_idx += 1;
    }
}

fn execute_ld_r_hl<'a, B>(
    context: &'a Context,
    cpu: &mut Cpu,
    bus: &B,
) -> Result<&'a Context, CycleError>
where
    B: ReadableFromPc,
{
    let addr = cpu.registers.get_hl();
    let integer = bus.read_in_cycle(addr)?;
    let reg = set_r_register_from_opcode(context.opcode, integer, cpu)
        .map_err(CycleError::UnsupportedInstruction)?;
    debug!(
        "{}, LD {:?}, (HL) -> {}",
        context.opcode.to_hex_string(),
        reg,
        integer.to_hex_string()
    );
    Ok(context)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::bus::BusError;
//     use crate::z80::Cpu;
//     use std::cell::Cell;
//
//     struct StubBus {
//         data: u8,
//         fail_read: bool,
//
//         last_addr_read: Cell<u16>,
//     }
//
//     impl Readable for StubBus {
//         fn read(&self, _addr: u16) -> Result<u8, BusError> {
//             if self.fail_read {
//                 Err(BusError::AccessError)
//             } else {
//                 Ok(self.data)
//             }
//         }
//     }
//
//     impl ReadableFromPc for StubBus {
//         fn read_from_pc(&self, _cpu: &mut Cpu) -> Result<u8, CycleError> {
//             if self.fail_read {
//                 Err(CycleError::BusReadError(BusError::AccessError))
//             } else {
//                 Ok(self.data)
//             }
//         }
//
//         fn read_in_cycle(&self, addr: u16) -> Result<u8, CycleError> {
//             self.last_addr_read.set(addr);
//             self.read(addr).map_err(CycleError::BusReadError)
//         }
//     }
//
//     #[test]
//     fn test_execute_ld_a_hl() {
//         let mut cpu = Cpu::new();
//         cpu.registers.set_h(0x12);
//         cpu.registers.set_l(0x34);
//         let bus = MockBus {
//             data: 0x42,
//             fail_read: false,
//         };
//         let context = Context { opcode: 0x7E }; // LD A, (HL)
//
//         let result = execute_ld_r_hl(&context, &mut cpu, &bus);
//
//         assert!(result.is_ok());
//         assert_eq!(cpu.registers.get_a(), 0x42);
//     }
//
//     #[test]
//     fn test_execute_ld_b_hl() {
//         let mut cpu = Cpu::new();
//         cpu.registers.set_hl(0x5678);
//         let bus = MockBus {
//             data: 0xAB,
//             fail_read: false,
//         };
//         let context = Context { opcode: 0x46 }; // LD B, (HL)
//
//         let result = execute_ld_r_hl(&context, &mut cpu, &bus);
//
//         assert!(result.is_ok());
//         assert_eq!(cpu.registers.get_b(), 0xAB);
//     }
//
//     #[test]
//     fn test_execute_ld_all_regs_hl() {
//         let test_cases = vec![
//             (0x4E, 0xCD, |cpu: &Cpu| cpu.registers.get_c()), // LD C, (HL)
//             (0x56, 0xEF, |cpu: &Cpu| cpu.registers.get_d()), // LD D, (HL)
//             (0x5E, 0x12, |cpu: &Cpu| cpu.registers.get_e()), // LD E, (HL)
//             (0x66, 0x34, |cpu: &Cpu| cpu.registers.get_h()), // LD H, (HL)
//             (0x6E, 0x56, |cpu: &Cpu| cpu.registers.get_l()), // LD L, (HL)
//         ];
//
//         for (opcode, expected_value, get_reg) in test_cases {
//             let mut cpu = Cpu::new();
//             cpu.registers.set_hl(0x9ABC);
//             let bus = MockBus {
//                 data: expected_value,
//                 fail_read: false,
//             };
//             let context = Context { opcode };
//
//             let result = execute_ld_r_hl(&context, &mut cpu, &bus);
//
//             assert!(result.is_ok());
//             assert_eq!(get_reg(&cpu), expected_value);
//         }
//     }
//
//     #[test]
//     fn test_execute_ld_r_hl_bus_error() {
//         let mut cpu = Cpu::new();
//         cpu.registers.set_hl(0x1234);
//         let bus = MockBus {
//             data: 0x00,
//             fail_read: true,
//         };
//         let context = Context { opcode: 0x7E }; // LD A, (HL)
//
//         let result = execute_ld_r_hl(&context, &mut cpu, &bus);
//
//         assert!(result.is_err());
//         assert!(matches!(result, Err(CycleError::BusReadError(_))));
//     }
// }
