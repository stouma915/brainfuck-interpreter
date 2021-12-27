use std::collections::HashMap;
use std::io;
use std::io::Write;

use colored::Colorize;

use crate::ascii_converter;
use crate::memory::Memory;
use crate::util;

pub struct Data {
    pub content: String,
    pub memory: Memory,
}

pub struct BFError {
    pub message: String,
}

pub fn eval(code: &String, memory: &mut Memory) -> Result<Data, BFError> {
    let mut result = String::from("");
    let mut error = None;

    let count_of_start_loop = util::count(&code, |c| c == '[');
    let count_of_end_loop = util::count(&code, |c| c == ']');

    if count_of_start_loop != count_of_end_loop {
        error = Some(BFError {
            message: String::from("Syntax Error"),
        });
    }

    let mut index = 0;
    let mut stop = false;

    code.chars().for_each(|c| {
        if error.is_none() && !stop {
            match c {
                '+' => memory.increment_value(),
                '-' => memory.decrement_value(),
                '>' => memory.increment(),
                '<' => memory.decrement(),
                '.' => {
                    let char_code = memory.get_content();
                    let char = ascii_converter::convert_to_char(char_code);
                    match char {
                        Some(c) => result.push_str(c.to_string().as_str()),
                        None => {
                            error = Some(BFError {
                                message: String::from(format!(
                                    "Unknown character code: {}",
                                    char_code
                                )),
                            })
                        }
                    }
                }
                ',' => {
                    println!("Input was requested.");

                    let mut done = false;
                    let mut input = 0i16;

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
                            }
                            Err(_) => {
                                println!("{}", "Please enter a 1 byte number.".bright_red());
                            }
                        }
                    }

                    memory.set_value(input);
                }
                '[' => {
                    let code_before_bracket = code[0..index].parse::<String>().unwrap();
                    let code_after_bracket = code[index..code.len()].parse::<String>().unwrap();

                    let mut count_of_bracket = 0;
                    let mut count_of_closing_bracket = 0;

                    let mut loop_end_index = 0;
                    let mut found = false;

                    let chars_after_bracket = code_after_bracket.chars().collect::<Vec<char>>();
                    chars_after_bracket.iter().enumerate().for_each(|elem| {
                        let i = elem.0;
                        let value = elem.1;

                        if !found {
                            if *value == '[' {
                                count_of_bracket = count_of_bracket + 1;
                            }
                            if *value == ']' {
                                count_of_closing_bracket = count_of_closing_bracket + 1;
                            }

                            if count_of_bracket == count_of_closing_bracket {
                                loop_end_index = i + code_before_bracket.len();
                                found = true;
                            }
                        }
                    });

                    if !found {
                        error = Some(BFError {
                            message: String::from("The end of the loop couldn't be identified."),
                        });
                    } else {
                        let length_of_code = code.len();
                        let content_of_loop =
                            code[index + 1..loop_end_index].parse::<String>().unwrap();
                        let after_loop = code[loop_end_index + 1..length_of_code]
                            .parse::<String>()
                            .unwrap();

                        while memory.get_content() != 0 {
                            let run_result = eval(&content_of_loop, memory);
                            match run_result {
                                Ok(data) => {
                                    result.push_str(data.content.as_str());
                                }
                                Err(err) => {
                                    error = Some(err);
                                    break;
                                }
                            }
                        }

                        let run_result = eval(&after_loop, memory);
                        match run_result {
                            Ok(data) => {
                                result.push_str(data.content.as_str());
                            }
                            Err(err) => {
                                error = Some(err);
                            }
                        }
                        stop = true;
                    }
                }
                _ => {}
            }
        }

        index = index + 1;
    });

    let immutable_memory = Memory {
        pointer: memory.pointer,
        content: HashMap::from_iter(memory.get_contents()),
    };

    match error {
        Some(error) => Err(error),
        None => Ok(Data {
            content: result,
            memory: immutable_memory,
        }),
    }
}
