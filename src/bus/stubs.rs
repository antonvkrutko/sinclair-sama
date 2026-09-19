use crate::bus::{BusError, Readable};
use std::cell::Cell;

pub struct StubBus {
    pub data: u8,
    pub fail_read: bool,

    pub last_addr_read: Cell<u16>,
}

impl Readable for StubBus {
    fn read(&self, addr: u16) -> Result<u8, BusError> {
        self.last_addr_read.set(addr);
        if self.fail_read {
            Err(BusError::AccessError)
        } else {
            Ok(self.data)
        }
    }
}
