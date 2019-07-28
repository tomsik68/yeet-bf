extern crate inkwell;

use llvm_compiler::brainf_to_llvm::compile_brainf;
use std::env;
use std::path::Path;

mod bf;
mod input;
mod interp;
mod llvm_compiler;
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
    //interp::interpret(&prog, input);
    let module = compile_brainf(prog).expect("Failed LLVM compilation");
    module
        .print_to_file("module.ll")
        .expect("Failed to print bitcode");
    module.write_bitcode_to_path(Path::new("module.bc"));
}
