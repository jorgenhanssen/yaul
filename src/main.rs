mod args;
mod instructions;
mod parser;
mod runner;
mod syscall;

use args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut parser = parser::Parser::new(args.file);

    let compile_start_time = std::time::Instant::now();

    let instructions = match parser.get_instructions() {
        Ok(instructions) => instructions,
        Err(e) => panic!("Failed to parse instructions: {}", e),
    };

    let compile_time = compile_start_time.elapsed();
    println!("Compiled in {:?}", compile_time);

    let mut runner = runner::Runner::new(args.registers as usize);
    if args.fast {
        runner.run::<true>(&instructions);
    } else {
        runner.run::<false>(&instructions);
    }
}
