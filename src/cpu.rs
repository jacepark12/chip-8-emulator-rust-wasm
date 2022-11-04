use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::display::Display;

type Bit = bool;

const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_WIDTH: usize = 64;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct CPU {
    position_in_memory: usize,
    index_register: u16,
    registers: [u8; 16],
    #[serde(with = "BigArray")]
    memory: [u8; 0x1000],
    #[serde(with = "BigArray")]
    frame_buffer: [Bit; 2048],
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn print_to_console(&self) {
        Display::print_framebuffer(self.frame_buffer);
    }

    pub fn load_rom(&mut self, path: &str) {
        const BYTES_PER_LINE: usize = 16;

        let mut mem_pos = 0x200;
        let mut f = File::open(path).expect("Unable to open file.");
        let mut f_pos = 0;
        let mut buffer = [0; BYTES_PER_LINE];

        loop {
            let read_bytes = f.read(&mut buffer).unwrap();

            for i in 0..read_bytes {
                self.memory[mem_pos] = buffer[i];
                mem_pos += 1;
            }

            f_pos += BYTES_PER_LINE;
            let seek_result = f.seek(SeekFrom::Start(f_pos.try_into().unwrap()));

            match seek_result {
                Err(_) => break,
                _ => (),
            }

            if read_bytes == 0 {
                break;
            }
        }
    }
}

#[wasm_bindgen]
impl CPU {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut _cpu = CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            index_register: 0,
            frame_buffer: [false; 2048],
            delay_timer: 0,
            sound_timer: 0,
        };

        let sprite_data = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, data) in sprite_data.iter().enumerate() {
            _cpu.memory[i] = *data;
        }

        _cpu
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_xkk(&mut self, x: u8, kk: u8) {
        let arg1 = self.registers[x as usize];

        let (val, overflow) = arg1.overflowing_add(kk);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn sub_xy(&mut self, vx: u8, vy: u8) {
        let arg1 = self.registers[vx as usize];
        let arg2 = self.registers[vy as usize];
        let (val, underflow) = arg1.overflowing_sub(arg2);

        self.registers[vx as usize] = val;

        if underflow {
            self.registers[0xF] = 0;
        } else {
            self.registers[0xF] = 1;
        }
    }

    fn subn_xy(&mut self, vx: u8, vy: u8) {
        let arg1 = self.registers[vy as usize];
        let arg2 = self.registers[vx as usize];
        let (val, underflow) = arg1.overflowing_sub(arg2);

        self.registers[vx as usize] = val;

        if underflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn se(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] == kk {
            self.position_in_memory += 2;
        }
    }

    fn sne(&mut self, vx: u8, kk: u8) {
        if self.registers[vx as usize] != kk {
            self.position_in_memory += 2;
        }
    }

    fn sne_xy(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] != self.registers[vy as usize] {
            self.position_in_memory += 2;
        }
    }

    fn se_registers(&mut self, vx: u8, vy: u8) {
        if self.registers[vx as usize] == self.registers[vy as usize] {
            self.position_in_memory += 2;
        }
    }

    fn ld(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vy as usize];
    }

    fn or(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vx as usize] | self.registers[vy as usize];
    }

    fn and(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vx as usize] & self.registers[vy as usize];
    }

    fn xor(&mut self, vx: u8, vy: u8) {
        self.registers[vx as usize] = self.registers[vx as usize] ^ self.registers[vy as usize];
    }

    fn shr(&mut self, vx: u8) {
        let val = self.registers[vx as usize] >> 1;

        self.registers[vx as usize] = val;

        if val & 0x1 == 1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn shl(&mut self, vx: u8) {
        let val = self.registers[vx as usize] << 1;

        self.registers[vx as usize] = val;

        if val & 0x80 == 1 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack oveflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn jump(&mut self, nnn: u16) {
        self.position_in_memory = nnn as usize;
    }

    fn jump_v0(&mut self, nnn: u16) {
        self.position_in_memory = (nnn + self.registers[0x0] as u16) as usize;
    }

    fn set_register(&mut self, vx: u8, value: u8) {
        self.registers[vx as usize] = value;
    }

    fn set_vx_dt(&mut self, vx: u8) {
        self.registers[vx as usize] = self.delay_timer;
    }

    fn set_dt_vx(&mut self, vx: u8) {
        self.delay_timer = self.registers[vx as usize];
    }

    fn set_st_vx(&mut self, vx: u8) {
        self.sound_timer = self.registers[vx as usize];
    }

    fn fx55(&mut self, vx: u8) {
        self.memory[(self.index_register as usize)..(self.index_register + vx as u16) as usize]
            .copy_from_slice(&self.registers[0..(vx as usize)]);

        self.index_register += vx as u16;
        self.index_register += 1;
    }

    fn fx65(&mut self, vx: u8) {
        self.memory[(self.index_register as usize)..(self.index_register + vx as u16) as usize]
            .copy_from_slice(&self.registers[0..(vx as usize)]);

        self.index_register += vx as u16;
        self.index_register += 1;
    }

    fn fx33(&mut self, vx: u8) {
        let x = self.registers[vx as usize];

        self.memory[self.index_register as usize] = x / 100;
        self.memory[self.index_register as usize + 1] = (x / 10) % 10;
        self.memory[self.index_register as usize + 2] = (x % 100) % 10;
    }

    fn display(&mut self, vx: usize, vy: usize, n: u8) {
        let x = self.registers[vx];
        let y = self.registers[vy];

        for i in 0..n {
            let mut decode_target = self.memory[(self.index_register + i as u16) as usize];

            for j in 0..8 {
                let mask_result = if decode_target & 0x1 == 1 {
                    true
                } else {
                    false
                };
                decode_target = decode_target >> 1;

                // 1. Get x, y coordinate.
                let xi = (x + (7 - j)) as usize % DISPLAY_WIDTH;
                let yi = (y + i) as usize % DISPLAY_HEIGHT;

                // 2. Get index of frame buffer based on cooridnate.
                let frame_buffer_index = yi * 64 + xi;

                if frame_buffer_index >= 2048 {
                    println!("frame buffer index exceeded")
                }

                let buf = &self.frame_buffer[frame_buffer_index];

                match buf ^ mask_result {
                    true => self.registers[0xF as usize] = 1,
                    false => self.registers[0xF as usize] = 0,
                }

                self.frame_buffer[frame_buffer_index] = mask_result;
            }
        }
    }

    fn clear_display(&mut self) {
        self.frame_buffer = [false; 2048];
    }

    pub fn run_cycle(&mut self) {
        let opcode = self.read_opcode();

        // Since opcode is u16, position_in_memory increases with 2
        self.position_in_memory += 2;

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = ((opcode & 0x000F) >> 0) as u8;

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;

        match (c, x, y, d) {
            (0, 0, 0, 0) => {
                return;
            }
            (0, 0, 0xE, 0) => self.clear_display(),
            (0, 0, 0xE, 0xE) => self.ret(),
            (1, _, _, _) => self.jump(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se(x, kk),
            (0x4, _, _, _) => self.sne(x, kk),
            (0x5, _, _, _) => self.se_registers(x, y),
            (0x6, _, _, _) => self.set_register(x, kk),
            (0x7, _, _, _) => self.add_xkk(x, kk),
            (0x8, _, _, 0x0) => self.ld(x, y),
            (0x8, _, _, 0x1) => self.or(x, y),
            (0x8, _, _, 0x2) => self.and(x, y),
            (0x8, _, _, 0x3) => self.xor(x, y),
            (0x8, _, _, 0x4) => self.add_xy(x, y),
            (0x8, _, _, 0x5) => self.sub_xy(x, y),
            (0x8, _, _, 0x6) => self.shr(x),
            (0x8, _, _, 0x7) => self.subn_xy(x, y),
            (0x8, _, _, 0xE) => self.shl(x),
            (0x9, _, _, 0x0) => self.sne_xy(x, y),
            (0xA, _, _, _) => self.index_register = nnn,
            (0xB, _, _, _) => self.jump_v0(nnn),
            (0xD, _, _, _) => self.display(x as usize, y as usize, d),
            (0xF, _, 0x0, 0x7) => self.set_vx_dt(x),
            (0xF, _, 0x1, 0x5) => self.set_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.set_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.index_register += self.registers[x as usize] as u16,
            (0xF, _, 0x3, 0x3) => self.fx33(x),
            (0xF, _, 0x5, 0x5) => self.fx55(x),
            (0xF, _, 0x6, 0x5) => self.fx65(x),
            _ => todo!("opcode {:04x}", opcode),
        }
    }
}
