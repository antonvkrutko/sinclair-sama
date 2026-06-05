mod bus;
mod z80;

use bus::Bus;
use z80::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    bus.load_rom_raw(&[
        0x00,
        0x3E, 0xAF,
    ]);

    println!("Loaded rom");
    bus.dump_rom();

    loop {
        match cpu.cycle(&bus) {
            Ok(_) => {
                cpu.dump()
            }
            Err(cycle_error) => {
                println!("CPU cycle error: {:?}, exiting...", cycle_error);
                break;
            }
        }
    }
}
