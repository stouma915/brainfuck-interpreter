use crate::ascii_converter;
use crate::memory::Memory;
use crate::util;
use std::io;
use std::io::Write;
use colored::Colorize;

pub struct Data {
    pub content: String,
    pub memory: Memory
}

pub struct BFError {
    pub message: String
}

pub fn run(code: String) -> Result<Data, BFError> {
    let mut result = String::from("");
    let mut memory = Memory::new();
    let mut error = None;

    let count_of_start_loop = util::count(&code, |c| c == '[');
    let count_of_end_loop = util::count(&code, |c| c == ']');

    if count_of_start_loop != count_of_end_loop {
        error = Some(BFError { message: String::from("Syntax Error") } );
    }

    code.chars().for_each(|c| {
        if error.is_none() {
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
                ',' => {
                    println!("Input was requested.");

                    let mut done = false;
                    let mut input = 0 as i16;

                    while !done {
                        println!();
                        print!("{}> ", "Input".bright_blue());
                        io::stdout().flush().unwrap();

                        let mut word = String::new();
                        io::stdin().read_line(&mut word).ok();

                        let trimmed = word.replace("\n", "");
                        let parsed = trimmed.parse::<i16>();
                        match parsed {
                            Ok(result) => {
                                if result >= -128 && result <= 127 {
                                    input = result;
                                    done = true;
                                } else {
                                    println!("{}", "Please enter a 1 byte number.".bright_red());
                                }
                            },
                            Err(_) => {
                                println!("{}", "Please enter a 1 byte number.".bright_red());
                            }
                        }
                    }

                    memory.set_value(input);
                },
                _ => {}
            }
        }
    });

    return match error {
        Some(error) => Err(error),
        None => Ok(Data { content: result, memory })
    };
}