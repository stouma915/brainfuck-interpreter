use std::time::{SystemTime, UNIX_EPOCH};

use crate::memory::Memory;

pub fn parse_memory(mut memory: Memory) -> String {
    if memory.get_contents().is_empty() {
        return String::from("[]");
    }

    let mut parsed_memory = String::from("[");

    let memory_contents = memory.get_contents();

    let mut keys = memory_contents.keys().collect::<Vec<&i32>>();
    keys.sort();
    for key in keys {
        let value = memory_contents.get(key).unwrap();
        parsed_memory.push_str(&*format!("{} : {}, ", key, value));
    }

    parsed_memory = parsed_memory[0..parsed_memory.len() - 2].parse().unwrap();
    parsed_memory.push_str("]");

    parsed_memory
}

pub fn current_epoch_milli() -> Option<u128> {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => Some(duration.as_millis()),
        Err(_) => None,
    }
}

pub fn search_loop_end(before: &str, after: &str) -> Option<usize> {
    let mut result = None;

    let mut count_of_bracket = 0;
    let mut count_of_closing_bracket = 0;

    for (index, char) in after.chars().enumerate() {
        if char == '[' {
            count_of_bracket += 1;
        } else if char == ']' {
            count_of_closing_bracket += 1;
        }

        if count_of_bracket == count_of_closing_bracket {
            result = Some(index + before.len());
            break;
        }
    }

    result
}
