use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Keypad {
    keys: [u8; 16],
}

#[wasm_bindgen]
impl Keypad {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut keys = [0; 16];
        Keypad { keys: keys }
    }

    pub fn key_down(&mut self, key: char) {
        match key {
            '1' => self.keys[0x1] = 1,
            '2' => self.keys[0x2] = 1,
            '3' => self.keys[0x3] = 1,
            'C' => self.keys[0xC] = 1,
            '4' => self.keys[0x4] = 1,
            '5' => self.keys[0x5] = 1,
            '6' => self.keys[0x6] = 1,
            'D' => self.keys[0xD] = 1,
            '7' => self.keys[0x7] = 1,
            '8' => self.keys[0x8] = 1,
            '9' => self.keys[0x9] = 1,
            'E' => self.keys[0xE] = 1,
            'A' => self.keys[0xA] = 1,
            '0' => self.keys[0x0] = 1,
            'B' => self.keys[0xB] = 1,
            'F' => self.keys[0xF] = 1,
            _ => return,
        }
    }

    pub fn key_up(&mut self, key: char) {
        match key {
            '1' => self.keys[0x1] = 0,
            '2' => self.keys[0x2] = 0,
            '3' => self.keys[0x3] = 0,
            'C' => self.keys[0xC] = 0,
            '4' => self.keys[0x4] = 0,
            '5' => self.keys[0x5] = 0,
            '6' => self.keys[0x6] = 0,
            'D' => self.keys[0xD] = 0,
            '7' => self.keys[0x7] = 0,
            '8' => self.keys[0x8] = 0,
            '9' => self.keys[0x9] = 0,
            'E' => self.keys[0xE] = 0,
            'A' => self.keys[0xA] = 0,
            '0' => self.keys[0x0] = 0,
            'B' => self.keys[0xB] = 0,
            'F' => self.keys[0xF] = 0,
            _ => return,
        }
    }

    pub fn key_state(&self, key: u8) -> u8 {
        match key {
            0x1 => self.keys[0x1],
            0x2 => self.keys[0x2],
            0x3 => self.keys[0x3],
            0xC => self.keys[0xC],
            0x4 => self.keys[0x4],
            0x5 => self.keys[0x5],
            0x6 => self.keys[0x6],
            0xD => self.keys[0xD],
            0x7 => self.keys[0x7],
            0x8 => self.keys[0x8],
            0x9 => self.keys[0x9],
            0xE => self.keys[0xE],
            0xA => self.keys[0xA],
            0x0 => self.keys[0x0],
            0xB => self.keys[0xB],
            0xF => self.keys[0xF],
            _ => 0,
        }
    }

    pub fn get_down_key(&self) -> Option<u8> {
        for (i, key_state) in self.keys.iter().enumerate() {
            if *key_state == 1 as u8 {
                return Some(i as u8);
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_down() {
        let mut keypad = Keypad::new();

        keypad.key_down('2');

        assert_eq!(keypad.key_state(2), 1)
    }

    #[test]
    fn test_key_up() {
        let mut keypad = Keypad::new();

        keypad.key_up('0');

        assert_eq!(keypad.key_state(0x0), 0)
    }

    #[test]
    fn test_get_down_key() {
        let mut keypad = Keypad::new();

        assert_eq!(keypad.get_down_key(), None);

        keypad.key_down('C');

        assert_eq!(keypad.get_down_key(), Some(0xC));

        keypad.key_up('C');

        assert_eq!(keypad.get_down_key(), None);
    }
}
