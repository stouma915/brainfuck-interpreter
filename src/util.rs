use crate::memory::Memory;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn parse_memory(mut memory: Memory) -> String {
    if memory.get_contents().is_empty() {
        return String::from("[]");
    }

    let mut parsed_memory = String::from("[");

    let memory_contents = memory.get_contents();

    memory_contents
        .keys()
        .collect::<Vec<&i32>>()
        .iter()
        .for_each(|key| {
            let value = memory_contents.get(key).unwrap();
            parsed_memory.push_str(&*format!("{} : {}, ", key, value));
        });

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
