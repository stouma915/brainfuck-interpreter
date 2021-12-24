use std::io;
use std::io::Write;
use colored::Colorize;

mod ascii_converter;
mod interpreter;
mod memory;
mod util;

fn main() {
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
            let run_result = interpreter::run(trimmed);
            match run_result {
                Ok(data) => {
                    println!();
                    println!("{}", "Execution successful.".bright_green());
                    println!("Content: {}", data.content);
                    println!("Memory: {}", util::parse_memory(data.memory));
                },
                Err(error) => {
                    println!();
                    println!("{}", "Execution failed.".bright_red());
                    println!("Message: {}", error.message);
                }
            }
        }
    }
}
