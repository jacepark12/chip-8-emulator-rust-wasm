use core::time;
use std::{env, thread};

mod cpu;
mod display;

fn main() {
    let mut _cpu = cpu::CPU::new();

    let mut path = match env::current_dir() {
        Ok(result) => result,
        Err(_err) => panic!(),
    };
    path.push("rom/IBM_Logo.ch8");
    _cpu.load_rom(path.to_str().unwrap());

    // run
    loop {
        _cpu.run_cycle();
        _cpu.print_to_console();
        thread::sleep(time::Duration::from_millis(16));
    }

    // TODO : Implement loop break.
    // Currently, process has to be terminated manually.
}
