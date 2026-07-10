use crate::bus::Bus;
use crate::z80::{Cpu, CycleError};

pub trait ReadableFromPc {
    fn read_from_pc(&self, cpu: &mut Cpu) -> Result<u8, CycleError>;
    fn read_in_cycle(&self, addr: u16) -> Result<u8, CycleError>;
}

impl ReadableFromPc for Bus {
    fn read_from_pc(&self, cpu: &mut Cpu) -> Result<u8, CycleError> {
        let value = self.read_in_cycle(cpu.registers.current_pc())?;
        cpu.registers.increment_pc();
        Ok(value)
    }

    fn read_in_cycle(&self, addr: u16) -> Result<u8, CycleError> {
        let value = self.read(addr).map_err(CycleError::BusReadError)?;
        Ok(value)
    }
}
