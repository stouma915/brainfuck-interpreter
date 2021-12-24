use crate::ascii_converter;
use crate::memory::Memory;

pub struct RunResult {
    pub content: String,
    pub memory: Memory
}

pub fn run(code: String) -> RunResult {
    let mut result = String::from("");
    let mut memory = Memory::new();

    code.chars().for_each(|c| {
        match c {
            '+' => memory.increment_value(),
            '-' => memory.decrement_value(),
            '>' => memory.increment(),
            '<' => memory.decrement(),
            '.' => {
                let char = ascii_converter::convert_to_char(memory.get_content());
                match char {
                    Some(c) => result.push_str(c.to_string().as_str()),
                    None => result.push_str("?")
                }
            },
            _ => {}
        }
    });

    return RunResult {
        content: result,
        memory
    };
}