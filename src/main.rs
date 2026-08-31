mod bus;
mod z80;
mod utils;

use log::{debug, error, info};
use bus::Bus;
use z80::Cpu;

fn main() {
    env_logger::init();
    info!("Starting up...");

    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    bus.load_rom_raw(&[
        0x00,
        0x3E, 0xAF,
        0x0E, 0xDD,
        0x06, 0x23,
        0xFF
    ]);

    debug!("Loaded rom:");
    bus.dump_rom();

    loop {
        match cpu.cycle(&bus) {
            Ok(_) => {
                cpu.dump();
            }
            Err(cycle_error) => {
                error!("CPU cycle error: {:?}, exiting...", cycle_error);
                break;
            }
        }
    }
}
