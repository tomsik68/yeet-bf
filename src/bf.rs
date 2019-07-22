#[derive(Debug)]
pub enum BfInst {
    Inc,
    Dec,

    IncPtr,
    DecPtr,

    Read,
    Write,

    LoopStart,
    LoopEnd,
}
