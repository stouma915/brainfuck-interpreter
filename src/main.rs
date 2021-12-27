use std::io::Write;
use std::process::exit;
use std::{fs, io};

use clap::{App, Arg};
use colored::Colorize;

use crate::memory::Memory;

mod ascii_converter;
mod interpreter;
mod memory;
mod util;

fn main() {
    let matches = App::new("Brainf**k Interpreter")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("1.0.0"))
        .arg(
            Arg::with_name("SOURCE")
                .help("Brainfuck source file (Run interactive if not specified)"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enable verbose"),
        )
        .get_matches();

    let verbose = matches.is_present("verbose");

    if let Some(source) = matches.value_of("SOURCE") {
        match fs::read_to_string(source) {
            Ok(content) => {
                exit(interpreter(content, verbose));
            }
            Err(err) => {
                println!("Unable to read source file: {:?}", err.kind());
                exit(1);
            }
        }
    } else {
        exit(interactive_interpreter());
    }
}

fn interpreter(source_code: String, verbose: bool) -> i32 {
    let mut memory = Memory::new();
    let run_result = interpreter::eval(&source_code, &mut memory);

    match run_result {
        Ok(data) => {
            if verbose {
                println!("{}", "Execution successful.".bright_green());
                println!("Content: {}", data.content);
                println!("Memory: {}", util::parse_memory(data.memory));
            } else {
                println!("{}", data.content);
            }

            0
        }
        Err(error) => {
            println!("{}", "Execution failed.".bright_red());
            println!("Message: {}", error.message);

            1
        }
    }
}

fn interactive_interpreter() -> i32 {
    let mut exit = false;

    println!("Entering interactive interpreter.");
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
            let run_result = interpreter::eval(&trimmed, &mut memory);

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

    0
}
