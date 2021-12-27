use colored::Colorize;
use std::io::Write;
use std::process::exit;
use std::{env, io};

use crate::memory::Memory;

mod ascii_converter;
mod interpreter;
mod memory;
mod util;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.is_empty() {
        exit(interactive_interpreter());
    }
}

fn interactive_interpreter() -> i32 {
    let mut exit = false;

    println!("Type \"exit\" to exit.");

    while !exit {
        println!();
        print!("{}> ", "Code".green());
        io::stdout().flush().unwrap();

        let mut word = String::new();
        io::stdin().read_line(&mut word).ok();

        let trimmed = word.replace("\n", "");

        if trimmed.to_lowercase() == "exit" {
            println!("Exit");
            exit = true;
        } else {
            let mut memory = Memory::new();
            let run_result = interpreter::run(&trimmed, &mut memory);
            match run_result {
                Ok(data) => {
                    println!();
                    println!("{}", "Execution successful.".bright_green());
                    println!("Content: {}", data.content);
                    println!("Memory: {}", util::parse_memory(data.memory));
                }
                Err(error) => {
                    println!();
                    println!("{}", "Execution failed.".bright_red());
                    println!("Message: {}", error.message);
                }
            }
        }
    }

    return 0;
}
