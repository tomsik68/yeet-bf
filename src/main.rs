use std::env;

mod bf;
mod input;
mod interp;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprint!("Usage: {} [filename.bf]", args[0]);
        return;
    }

    let filename = &args[1];
    let prog = parser::parse_program(filename).expect("Failed parsing the program");

    let input = input::stdin();
    interp::interpret(&prog, input);
}
