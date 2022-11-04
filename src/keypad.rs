use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Keypad {
    keys: HashMap<char, bool>,
}

#[wasm_bindgen]
impl Keypad {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Keypad {
            keys: HashMap::from([
                ('1', false),
                ('2', false),
                ('3', false),
                ('4', false),
                ('5', false),
                ('6', false),
                ('7', false),
                ('8', false),
                ('9', false),
                ('0', false),
                ('A', false),
                ('B', false),
                ('C', false),
                ('D', false),
                ('E', false),
                ('F', false),
            ]),
        }
    }

    pub fn key_down(&mut self, key: char) {
        match self.keys.get_mut(&key) {
            Some(key_state) => *key_state = true,
            None => return,
        }
    }

    pub fn key_up(&mut self, key: char) {
        match self.keys.get_mut(&key) {
            Some(key_state) => *key_state = false,
            None => return,
        }
    }

    pub fn key_state(&self, key: char) -> bool {
        match self.keys.get(&key) {
            Some(key_state) => *key_state,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_key_down() {
        let mut keypad = Keypad::new();

        keypad.key_down('0');

        assert_eq!(keypad.key_state('0'), true)
    }

    #[test]
    fn test_key_up() {
        let mut keypad = Keypad::new();

        keypad.key_up('0');

        assert_eq!(keypad.key_state('0'), false)
    }
}
