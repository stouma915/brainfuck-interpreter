use std::io;
use std::io::Write;

use colored::Colorize;

use crate::ascii_converter;
use crate::memory::Memory;

#[cfg(test)]
mod tests {
    use crate::interpreter::eval;
    use crate::Memory;

    #[test]
    fn can_evaluate_brainfuck_code() {
        assert_eq!(
            eval(&String::from("----[---->+<]>++.+.+."), &mut Memory::new())
                .ok()
                .unwrap()
                .content,
            "ABC"
        );
        assert_eq!(
            eval(&String::from("-[----->+<]>--.+.+."), &mut Memory::new())
                .ok()
                .unwrap()
                .content,
            "123"
        );
    }

    #[test]
    fn can_throw_an_error() {
        assert_eq!(
            eval(&String::from("[[[[["), &mut Memory::new())
                .err()
                .is_some(),
            true
        );
        assert_eq!(
            eval(&String::from("-."), &mut Memory::new())
                .err()
                .is_some(),
            true
        );
    }
}

pub struct EvalResult {
    pub content: String,
    pub memory: Memory,
}

pub struct EvalError {
    pub message: String,
}

pub fn eval(code: &String, memory: &mut Memory) -> Result<EvalResult, EvalError> {
    let mut content = String::from("");
    let mut error = None;

    let chars = code.chars().collect::<Vec<char>>();
    for elem in chars.iter().enumerate() {
        if error.is_some() {
            break;
        }

        match elem.1 {
            '+' => memory.increment_value(),
            '-' => memory.decrement_value(),
            '>' => memory.increment(),
            '<' => memory.decrement(),
            '.' => {
                let char_code = memory.get_content();
                match ascii_converter::convert_to_char(char_code) {
                    Some(ch) => content.push_str(ch.to_string().as_str()),
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
                let code_before_bracket = code[0..elem.0].parse::<String>().unwrap();
                let code_after_bracket = code[elem.0..code.len()].parse::<String>().unwrap();

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
                let code_to_loop = code[elem.0 + 1..loop_end_index.unwrap()]
                    .parse::<String>()
                    .unwrap();
                let after_loop = code[loop_end_index.unwrap() + 1..length_of_code]
                    .parse::<String>()
                    .unwrap();

                while memory.get_content() != 0 {
                    let result = eval(&code_to_loop, memory);
                    match result {
                        Ok(eval_result) => {
                            content.push_str(eval_result.content.as_str());
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
                        content.push_str(eval_result.content.as_str());
                    }
                    Err(err) => {
                        error = Some(err);
                    }
                }

                break;
            }
            _ => {}
        }
    }

    match error {
        Some(error) => Err(error),
        None => Ok(EvalResult {
            content,
            memory: memory.copied(),
        }),
    }
}
