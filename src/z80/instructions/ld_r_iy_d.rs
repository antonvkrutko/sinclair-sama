use crate::bus::Readable;
use crate::utils::ToHexString;
use crate::z80::instructions::constants::REGS;
use crate::z80::instructions::utils::{r_reg_opcode, set_r_register_from_opcode};
use crate::z80::instructions::{Context, Cycles, Instruction};
use crate::z80::utils::{ReadableFromPc, ReadableInCycle};
use crate::z80::{Cpu, CycleError};
use log::debug;

const LD_R_IY_D_OPCODE_MASK: u8 = 0x46;
pub const fn build_ld_r_iy_d_set<B>(instructions: &mut [Instruction<B>])
where
    B: Readable,
{
    const LD_R_IY_D_CYCLES: Cycles = Cycles {
        m_cycles: 5,
        t_states: 19,
    };

    let mut reg_idx = 0;
    while reg_idx < REGS.len() {
        instructions[r_reg_opcode(REGS[reg_idx], LD_R_IY_D_OPCODE_MASK)] = Instruction {
            cycles: LD_R_IY_D_CYCLES,
            execute: execute_ld_r_iy_d,
        };
        reg_idx += 1;
    }
}

fn execute_ld_r_iy_d<'a, B>(
    context: &'a Context,
    cpu: &mut Cpu,
    bus: &B,
) -> Result<&'a Context, CycleError>
where
    B: Readable,
{
    let displacement = bus.read_from_pc(cpu)? as i8;
    let iy = cpu.registers.get_iy();
    let addr = iy.wrapping_add(displacement as u16);

    let value = bus.read_in_cycle(addr)?;
    let reg = set_r_register_from_opcode(context.opcode, value, cpu)
        .map_err(CycleError::UnsupportedInstruction)?;

    debug!(
        "{} LD {:?}, (IY+{}) -> {}",
        context.opcode.to_hex_string(),
        reg,
        displacement,
        value.to_hex_string()
    );

    Ok(context)
}

#[cfg(test)]
mod test {
    use std::cell::Cell;
    use assert_hex::assert_eq_hex;
    use crate::bus::stubs::StubBus;
    use crate::z80::Cpu;
    use crate::z80::instructions::constants::{Reg, REGS};
    use crate::z80::instructions::Context;
    use crate::z80::instructions::ld_r_iy_d::{execute_ld_r_iy_d, LD_R_IY_D_OPCODE_MASK};
    use crate::z80::instructions::utils::r_reg_opcode;

    #[test]
    fn test_execute_ld_r_iy_d_positive_d() {
        let data = 0x42;
        let addr = 0xABCDu16;
        let displacement = 0x01i8;

        let mut bus = StubBus::default();
        bus.data_map.insert(0x00, displacement as u8);
        bus.data_map.insert(addr.wrapping_add(displacement as u16), data);

        for reg in REGS {
            let mut cpu = Cpu::new();
            cpu.registers.set_iy(addr);

            let context = Context {
                opcode: u8::try_from(r_reg_opcode(reg, LD_R_IY_D_OPCODE_MASK)).unwrap(),
            };

            let result = execute_ld_r_iy_d(&context, &mut cpu, &bus);

            assert!(result.is_ok());
            match reg {
                Reg::A => {
                    assert_eq_hex!(cpu.registers.get_a(), data)
                }
                Reg::B => {
                    assert_eq_hex!(cpu.registers.get_b(), data)
                }
                Reg::C => {
                    assert_eq_hex!(cpu.registers.get_c(), data)
                }
                Reg::D => {
                    assert_eq_hex!(cpu.registers.get_d(), data)
                }
                Reg::E => {
                    assert_eq_hex!(cpu.registers.get_e(), data)
                }
                Reg::H => {
                    assert_eq_hex!(cpu.registers.get_h(), data)
                }
                Reg::L => {
                    assert_eq_hex!(cpu.registers.get_l(), data)
                }
            }
        }
    }
}

