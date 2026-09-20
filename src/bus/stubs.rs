use crate::bus::{BusError, Readable};
use std::collections::HashMap;

#[derive(Default)]
pub struct StubBus {
    pub data_map: HashMap<u16, u8>,
    pub fail_read: bool,
}

impl Readable for StubBus {
    fn read(&self, addr: u16) -> Result<u8, BusError> {
        if self.fail_read {
            Err(BusError::AccessError)
        } else {
            Ok(self.data_map.get(&addr).unwrap().clone())
        }
    }
}
