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
            println!();
            println!("Content: {}", run_result.content);
            println!("Memory: {}", util::parse_memory(run_result.memory));
        }
    }
}
