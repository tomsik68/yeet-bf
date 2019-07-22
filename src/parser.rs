use bf::BfInst;

use std::fs::File;
use std::io::prelude::*;
use std::io::Result;
use std::path::Path;
use std::vec::Vec;

fn read_whole_file<S: AsRef<Path>>(filename: S) -> Result<String> {
    let mut file = File::open(filename.as_ref())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn char_to_inst(c: char) -> Option<BfInst> {
    use bf::BfInst::*;

    match c {
        '>' => Some(IncPtr),
        '<' => Some(DecPtr),
        '+' => Some(Inc),
        '-' => Some(Dec),
        '.' => Some(Write),
        ',' => Some(Read),
        '[' => Some(LoopStart),
        ']' => Some(LoopEnd),
        _ => None,
    }
}

pub fn parse_program<S: AsRef<Path>>(filename: S) -> Result<Vec<BfInst>> {
    Ok(read_whole_file(filename)?
        .chars()
        .filter_map(char_to_inst)
        .collect())
}
