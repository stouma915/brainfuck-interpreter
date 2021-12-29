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
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
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

    exit(if let Some(source) = matches.value_of("SOURCE") {
        match fs::read_to_string(source) {
            Ok(content) => interpreter(content, verbose),
            Err(err) => {
                println!("Unable to read source file: {:?}", err.kind());
                1
            }
        }
    } else {
        interactive_interpreter(verbose)
    })
}

fn interpreter(source_code: String, verbose: bool) -> i32 {
    let mut memory = Memory::new();
    let result = interpreter::eval(&source_code, &mut memory);

    match result {
        Ok(eval_result) => {
            if verbose {
                let content_to_show = if eval_result.content.len() == 0 {
                    "Nothing to show.".bright_red().italic()
                } else {
                    eval_result.content.as_str().white()
                };

                println!("{}", "Execution successful.".bright_green());
                println!("Content: {}", content_to_show);
                println!("Memory: {}", util::parse_memory(eval_result.memory));
            } else {
                println!("{}", eval_result.content);
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

fn interactive_interpreter(verbose: bool) -> i32 {
    let mut exit = false;

    println!("Entering interactive interpreter.");
    println!("Type \"exit\" to exit.");

    while !exit {
        println!();
        print!("{}> ", "Code".green());
        io::stdout().flush().unwrap();

        let mut word = String::new();
        io::stdin().read_line(&mut word).ok();

        let source_code = word.replace("\n", "");

        if source_code.to_lowercase() == "exit" {
            println!("Exit");
            exit = true;
        } else {
            let mut memory = Memory::new();
            let result = interpreter::eval(&source_code, &mut memory);

            match result {
                Ok(eval_result) => {
                    if verbose {
                        let content_to_show = if eval_result.content.len() == 0 {
                            "Nothing to show.".bright_red().italic()
                        } else {
                            eval_result.content.as_str().white()
                        };

                        println!();
                        println!("{}", "Execution successful.".bright_green());
                        println!("Content: {}", content_to_show);
                        println!("Memory: {}", util::parse_memory(eval_result.memory));
                    } else {
                        println!("{}", eval_result.content);
                    }
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
