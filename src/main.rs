mod cli;
mod instructions;
mod parser;

use clap::Parser;
use cli::CLI;

fn main() {
    let args = CLI::parse();

    let mut parser = parser::Parser::new(args.file);

    let compile_start_time = std::time::Instant::now();

    let instructions = match parser.get_instructions() {
        Ok(instructions) => instructions,
        Err(e) => panic!("Failed to parse instructions: {}", e),
    };

    let compile_time = compile_start_time.elapsed();
    println!("Compiled in {}μs", compile_time.as_micros());

    for instruction in instructions {
        println!("{:?}", &instruction);
    }
}
