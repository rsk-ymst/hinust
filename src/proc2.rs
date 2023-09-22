use core::{arch::asm, cell::Cell};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProcState {
    RUNNABLE,
    UNUSED,
    STABLE,
}

type vaddr_t = i32;


#[derive(Clone, Debug)]
#[repr(C, align(32))]
pub struct Process {
    pub pid: isize,      // プロセスID
    pub state: ProcState,          // プロセスの状態
    pub sp: usize,   // コンテキストスイッチ時のスタックポインタ
    pub stack: [u8; 8192]
}
