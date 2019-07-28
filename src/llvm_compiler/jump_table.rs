use bf::BfInst;
use std::collections::HashMap;
use std::vec::Vec;

pub fn compute_jump_table(prog: &Vec<BfInst>) -> HashMap<usize, usize> {
    let mut result = HashMap::new();
    let mut stack = vec![];

    for (ref inst, pc) in prog.iter().zip(0..) {
        match inst {
            BfInst::LoopStart => {
                stack.push(pc);
            }
            BfInst::LoopEnd => {
                let start = stack
                    .pop()
                    .expect("compute_jump_table: unbalanced loops in program");
                result.insert(start, pc);
                result.insert(pc, start);
            }
            _ => {}
        }
    }

    result
}
