use core::time;
use std::{env, thread};

mod cpu;
mod display;
mod keypad;

fn main() {
    let mut _cpu = cpu::CPU::new();

    let mut path = match env::current_dir() {
        Ok(result) => result,
        Err(_err) => panic!(),
    };
    // path.push("rom/IBM_Logo.ch8");
    // path.push("rom/test_opcode.ch8");
    path.push("rom/chip8-test-suite.ch8");
    _cpu.load_rom_from_path(path.to_str().unwrap());

    // run
    loop {
        // _cpu.keypad.key_up('0');
        _cpu.run_cycle();
        // _cpu.keypad.key_down('0');

        _cpu.print_to_console();
        thread::sleep(time::Duration::from_millis(4));
    }

    // TODO : Implement loop break.
    // Currently, process has to be terminated manually.
}
