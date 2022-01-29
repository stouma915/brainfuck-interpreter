use std::io;
use std::io::Write;

use colored::Colorize;

use crate::ascii_converter;
use crate::memory::Memory;
use crate::util;

pub struct EvalResult {
    pub output: String,
    pub memory: Memory,
}

pub struct EvalError {
    pub message: &'static str,
}

pub fn eval(code: &str, memory: &mut Memory) -> Result<EvalResult, EvalError> {
    let mut outputs: Vec<String> = Vec::new();
    let mut error = None;

    for (index, char) in code.chars().enumerate() {
        match char {
            '+' => memory.increment_value(),
            '-' => memory.decrement_value(),
            '>' => memory.increment(),
            '<' => memory.decrement(),
            '.' => match ascii_converter::convert(memory.get_current_value()) {
                Some(value) => outputs.push(String::from(value)),
                None => outputs.push(String::from("\0")),
            },
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
                    match trimmed.parse::<i16>() {
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
                let code_after_bracket = &code[index..code.len()].parse::<String>().unwrap();

                let loop_end_index = match util::search_loop_end(
                    code_before_bracket.as_str(),
                    code_after_bracket.as_str(),
                ) {
                    Some(index) => index,
                    None => {
                        error = Some("The end of the loop couldn't be identified.");
                        break;
                    }
                };

                let code_to_loop = code[index + 1..loop_end_index].parse::<String>().unwrap();
                let after_loop = code[loop_end_index + 1..code.len()]
                    .parse::<String>()
                    .unwrap();

                while memory.get_current_value() != 0 {
                    let result = eval(&code_to_loop, memory);
                    match result {
                        Ok(eval_result) => {
                            outputs.push(eval_result.output);
                        }
                        Err(err) => {
                            error = Some(err.message);
                            break;
                        }
                    }
                }

                let result = eval(&after_loop, memory);
                match result {
                    Ok(eval_result) => {
                        outputs.push(eval_result.output);
                    }
                    Err(err) => {
                        error = Some(err.message);
                    }
                }

                break;
            }
            _ => {}
        }
    }

    if let Some(err) = error {
        return Err(EvalError { message: err });
    }

    Ok(EvalResult {
        output: outputs.join(""),
        memory: memory.copied(),
    })
}
