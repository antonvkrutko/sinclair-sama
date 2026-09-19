use crate::bus::Readable;
use crate::utils::ToHexString;
use crate::z80::instructions::constants::REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Context, Cycles, Instruction};
use crate::z80::utils::ReadableInCycle;
use crate::z80::{Cpu, CycleError};
use log::debug;

// LD r, (HL)
const LD_R_HL_OPCODE_MASK: u8 = 0x46;

pub const fn build_ld_r_hl_set<B>(instructions: &mut [Instruction<B>])
where
    B: Readable,
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
    B: Readable,
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

#[cfg(test)]
mod test {
    use crate::bus::stubs::StubBus;
    use crate::z80::Cpu;
    use crate::z80::instructions::Context;
    use crate::z80::instructions::constants::{REGS, Reg};
    use crate::z80::instructions::ld_r_hl::{LD_R_HL_OPCODE_MASK, execute_ld_r_hl};
    use crate::z80::instructions::utils::r_reg_opcode;
    use assert_hex::assert_eq_hex;
    use std::cell::Cell;

    #[test]
    fn test_execute_ld_r_hl() {
        let data = 0x42;
        let bus = StubBus {
            data,
            fail_read: false,
            last_addr_read: Cell::default(),
        };

        for reg in REGS {
            let mut cpu = Cpu::new();
            let addr_arr: [u8; 2] = [0xAB, 0xCD];
            cpu.registers.set_h(addr_arr.get(0).unwrap().to_owned());
            cpu.registers.set_l(addr_arr.get(1).unwrap().to_owned());

            let context = Context {
                opcode: u8::try_from(r_reg_opcode(reg, LD_R_HL_OPCODE_MASK)).unwrap(),
            };

            let result = execute_ld_r_hl(&context, &mut cpu, &bus);

            assert!(result.is_ok());
            match reg {
                Reg::A => {
                    assert_eq!(cpu.registers.get_a(), data)
                }
                Reg::B => {
                    assert_eq!(cpu.registers.get_b(), data)
                }
                Reg::C => {
                    assert_eq!(cpu.registers.get_c(), data)
                }
                Reg::D => {
                    assert_eq!(cpu.registers.get_d(), data)
                }
                Reg::E => {
                    assert_eq!(cpu.registers.get_e(), data)
                }
                Reg::H => {
                    assert_eq!(cpu.registers.get_h(), data)
                }
                Reg::L => {
                    assert_eq!(cpu.registers.get_l(), data)
                }
            }
            assert_eq_hex!(
                bus.last_addr_read.get(),
                u16::from_be_bytes(addr_arr),
                "for reg: {:?}",
                reg
            );
        }
    }
}
