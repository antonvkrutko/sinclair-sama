#[derive(Debug)]
pub struct CpuRegisters {
    // general purpose registers
    bc: u16,
    de: u16,
    hl: u16,

    bc_: u16,
    de_: u16,
    hl_: u16,

    // special purpose registers
    pub pc: u16,
    sp: u16,

    // accumulators
    a: u8,
    a_: u8,

    // flags
    f: u8,
    f_: u8,

    ix: u16,
    iy: u16,
}

impl CpuRegisters {
    pub fn init() -> Self {
        CpuRegisters {
            bc: 0,
            de: 0,
            hl: 0,
            bc_: 0,
            de_: 0,
            hl_: 0,
            pc: 0,
            sp: 0,
            a: 0,
            a_: 0,
            f: 0,
            f_: 0,
            ix: 0,
            iy: 0,
        }
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn dump(&self) {
        println!("{:?}", self);
    }
}
