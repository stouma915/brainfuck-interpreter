use std::collections::{BTreeMap, HashMap};

pub struct Memory {
    pub pointer: i32,
    pub content: HashMap<i32, i16>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory {
            pointer: 0,
            content: HashMap::new()
        };
    }

    #[allow(mutable_borrow_reservation_conflict)]
    pub fn increment_value(&mut self) -> () {
        match self.content.get(&self.pointer) {
            Some(current_value) => {
                self.content.insert(self.pointer, current_value + 1);
            },
            None => {
                self.content.insert(self.pointer, 1);
            }
        }
    }

    #[allow(mutable_borrow_reservation_conflict)]
    pub fn decrement_value(&mut self) -> () {
        match self.content.get(&self.pointer) {
            Some(current_value) => {
                if current_value <= &(0 as i16) {
                    self.content.insert(self.pointer, 255);
                } else {
                    self.content.insert(self.pointer, current_value - 1);
                }
            },
            None => {
                self.content.insert(self.pointer, 255);
            }
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
        let new_pointer = self.pointer + 1;

        match self.content.get(&new_pointer) {
            None => {
                self.content.insert(new_pointer, 0);
            },
            _ => {}
        }

        self.pointer = new_pointer;
    }

    pub fn decrement(&mut self) -> () {
        let new_pointer =
            if self.pointer <= 0 {
                0
            } else {
                self.pointer - 1
            };

        match self.content.get(&new_pointer) {
            None => {
                self.content.insert(new_pointer, 0);
            },
            _ => {}
        }

        self.pointer = new_pointer;
    }

    pub fn get_content(&mut self) -> i16 {
        return match self.content.get(&self.pointer) {
            Some(content) => *content,
            None => 0 as i16
        }
    }

    pub fn get_contents(&mut self) -> BTreeMap<i32, i16> {
        let mut sorted_content = BTreeMap::new();

        let mut keys = self.content.keys().collect::<Vec<&i32>>();
        keys.sort();
        keys.iter().for_each(|key| {
            let value = self.content.get(key).unwrap();
            sorted_content.insert(**key, *value);
        });

        return sorted_content;
    }
}
