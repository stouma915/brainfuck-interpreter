use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

use colored::{ColoredString, Colorize};

use crate::memory::Memory;

pub fn parse_memory(memory: Memory) -> Vec<ColoredString> {
    let mut parsed_memories: Vec<ColoredString> = Vec::new();

    let memory_contents = memory.get_contents();

    let mut keys = memory_contents.keys().collect::<Vec<&i32>>();
    keys.sort();

    for key in keys {
        let value = memory_contents.get(key).unwrap().to_string();
        parsed_memories.push(
            ColoredString::from(
                format!(
                    "{} {} {}",
                    key.to_string().as_str().blue(),
                    ":".bright_white(),
                    value.as_str().white()
                )
                .as_str(),
            )
            .on_bright_black(),
        );
    }

    parsed_memories
}

pub fn current_epoch_milli() -> Result<u128, SystemTimeError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
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
