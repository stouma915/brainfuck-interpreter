use std::collections::HashMap;

pub struct Memory {
    pub pointer: i32,
    pub content: HashMap<i32, i16>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            pointer: 0,
            content: HashMap::new(),
        }
    }

    pub fn increment_value(&mut self) -> () {
        let new_value = self.content.get(&self.pointer).unwrap_or(&0i16) + 1;
        if new_value >= 256 {
            self.content.insert(self.pointer, 0);
        } else {
            self.content.insert(self.pointer, new_value);
        }
    }

    pub fn decrement_value(&mut self) -> () {
        let new_value = self.content.get(&self.pointer).unwrap_or(&256i16) - 1;
        if new_value <= -1 {
            self.content.insert(self.pointer, 255);
        } else {
            self.content.insert(self.pointer, new_value);
        }
    }

    pub fn set_value(&mut self, value: i16) -> () {
        self.content.insert(self.pointer, 0);

        (0..value.abs()).for_each(|_| {
            if value < 0 {
                self.decrement_value();
            } else if value > 0 {
                self.increment_value();
            }
        });
    }

    pub fn increment(&mut self) -> () {
        self.pointer += 1;
    }

    pub fn decrement(&mut self) -> () {
        if self.pointer >= 1 {
            self.pointer -= 1;
        }
    }

    pub fn get_content(&self) -> i16 {
        *self.content.get(&self.pointer).unwrap_or(&0i16)
    }

    pub fn get_contents(&self) -> HashMap<i32, i16> {
        self.content.clone()
    }

    pub fn copied(&self) -> Memory {
        Memory {
            pointer: self.pointer,
            content: self.content.clone(),
        }
    }
}
