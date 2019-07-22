use bf::BfInst;
use std::vec::Vec;

pub fn interpret(prog: &Vec<BfInst>) {
    use bf::BfInst::*;
    let mut memory = Vec::<u8>::with_capacity(65536);

    let mut pc: usize = 0;
    let mut ptr: usize = 0;

    loop {
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
                print!("{}", memory[ptr]);
            }

            LoopStart => {
                if memory[ptr] == 0 {
                    // TODO: go to next loop end
                    jumped = true;
                }
            }

            LoopEnd => {
                if memory[ptr] != 0 {
                    // TODO: go to previous loop start
                    jumped = true;
                }
            }
        }

        if !jumped {
            pc = pc + 1;
        }
    }
}
