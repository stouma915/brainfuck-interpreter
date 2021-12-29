use std::io;
use std::io::Write;

use colored::Colorize;

use crate::ascii_converter;
use crate::memory::Memory;
use crate::util;

pub struct EvalResult {
    pub content: String,
    pub memory: Memory,
}

pub struct EvalError {
    pub message: String,
}

pub fn eval(code: &String, memory: &mut Memory) -> Result<EvalResult, EvalError> {
    let mut result = String::from("");
    let mut error = None;

    let count_of_start_loop = util::count(&code, |c| c == '[');
    let count_of_end_loop = util::count(&code, |c| c == ']');

    if count_of_start_loop != count_of_end_loop {
        return Err(EvalError {
            message: String::from("Syntax Error"),
        });
    }

    let mut index = 0;

    let chars = code.chars().collect::<Vec<char>>();
    for c in chars {
        if error.is_some() {
            break;
        }

        match c {
            '+' => memory.increment_value(),
            '-' => memory.decrement_value(),
            '>' => memory.increment(),
            '<' => memory.decrement(),
            '.' => {
                let char_code = memory.get_content();
                match ascii_converter::convert_to_char(char_code) {
                    Some(ch) => result.push_str(ch.to_string().as_str()),
                    None => {
                        error = Some(EvalError {
                            message: String::from(format!("Unknown character code: {}", char_code)),
                        });
                        break;
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

                let mut loop_end_index = None;

                let chars_after_bracket = code_after_bracket.chars().collect::<Vec<char>>();
                for elem in chars_after_bracket.iter().enumerate() {
                    let i = elem.0;
                    let value = elem.1;

                    if *value == '[' {
                        count_of_bracket = count_of_bracket + 1;
                    }
                    if *value == ']' {
                        count_of_closing_bracket = count_of_closing_bracket + 1;
                    }

                    if count_of_bracket == count_of_closing_bracket {
                        loop_end_index = Some(i + code_before_bracket.len());
                        break;
                    }
                }

                if loop_end_index.is_none() {
                    error = Some(EvalError {
                        message: String::from("The end of the loop couldn't be identified."),
                    });
                    break;
                }

                let length_of_code = code.len();
                let code_to_loop = code[index + 1..loop_end_index.unwrap()]
                    .parse::<String>()
                    .unwrap();
                let after_loop = code[loop_end_index.unwrap() + 1..length_of_code]
                    .parse::<String>()
                    .unwrap();

                while memory.get_content() != 0 {
                    let result = eval(&code_to_loop, memory);
                    match result {
                        Ok(eval_result) => {
                            result.push_str(eval_result.content.as_str());
                        }
                        Err(err) => {
                            error = Some(err);
                            break;
                        }
                    }
                }

                let result = eval(&after_loop, memory);
                match result {
                    Ok(eval_result) => {
                        result.push_str(eval_result.content.as_str());
                    }
                    Err(err) => {
                        error = Some(err);
                    }
                }

                break;
            }
            _ => {}
        }

        index = index + 1;
    }

    match error {
        Some(error) => Err(error),
        None => Ok(EvalResult {
            content: result,
            memory: memory.copied(),
        }),
    }
}
