use bf::BfInst;
use std::vec::Vec;

pub fn interpret(prog: &Vec<BfInst>) {
    use bf::BfInst::*;
    let mut memory = vec![0u8; 65536];

    let mut pc: usize = 0;
    let mut ptr: usize = 0;

    while pc < prog.len() {
        let mut jumped = false;

        match prog[pc] {
            Inc => {
                memory[ptr] = memory[ptr] + 1;
            }

            Dec => {
                memory[ptr] = memory[ptr] - 1;
            }

            IncPtr => {
                ptr = ptr + 1;
            }

            DecPtr => {
                ptr = ptr - 1;
            }

            Read => {
                // TODO
                memory[ptr] = 0;
            }

            Write => {
                print!("{}", memory[ptr] as char);
            }

            LoopStart => {
                if memory[ptr] == 0 {
                    let mut in_loop = 1;
                    loop {
                        pc = pc + 1;
                        match prog[pc] {
                            LoopStart => {
                                in_loop += 1;
                            }
                            LoopEnd => {
                                in_loop -= 1;
                                if in_loop == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    assert!(pc < prog.len());
                    jumped = true;
                }
            }

            LoopEnd => {
                if memory[ptr] != 0 {
                    let mut in_loop = 1;
                    loop {
                        pc = pc - 1;
                        match prog[pc] {
                            LoopEnd => {
                                in_loop += 1;
                            }
                            LoopStart => {
                                in_loop -= 1;
                                if in_loop == 0 {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }

                    assert!(pc < prog.len());
                    jumped = true;
                }
            }
        }

        if !jumped {
            pc = pc + 1;
        }
    }
}
