use core::{arch::asm, cell::Cell};

use alloc::boxed::Box;

// use crate::proc::{Process, State};

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
    pub pid: Cell<i32>,      // プロセスID
    pub state: Cell<ProcState>,          // プロセスの状態
    pub sp: Cell<*mut i32>,   // コンテキストスイッチ時のスタックポインタ
    pub stack: Cell<[u8; 8192]>
}

// pub struct Task {
//     pub pid: i32,      // プロセスID
//     pub state: Cell<ProcState>,          // プロセスの状態
//     pub sp: i32,   // コンテキストスイッチ時のスタックポインタ
//     pub stack: Box<[u8; 8192]>
// }

unsafe impl Send for Process {}
unsafe impl Sync for Process {}


// static procs: [Process; 8] = [Process {
//     pid: -1,
//     state: State::PROC_UNUSED,
//     sp: -1,
//     stack: [0; 8192]
// }; 8];

// static proc_a: Process = Process {
//     pid: -1,
//     state: State::PROC_RUNNABLE,
//     sp: -1,
//     stack: [0; 8192]
// };

// static proc_b: Process = Process {
//     pid: -1,
//     state: State::PROC_RUNNABLE,
//     sp: -1,
//     stack: [0; 8192]
// };

// pub unsafe fn create_process(pc: i32) -> *mut Process {
//     for (i, proc) in procs.iter_mut().enumerate() {
//         if proc.state == State::PROC_UNUSED {
//             let sp: *mut i32 = proc.stack.as_mut_ptr().cast::<i32>();

//             for i in 0..13 {
//                 *sp.offset(-i) = 0;
//                 if i == 12 {
//                     *sp.offset(-i) = pc;
//                 }
//             }

//             proc.pid = (i + 1) as i32;
//             proc.state = State::PROC_RUNNABLE;
//             proc.sp = *sp;

//             return proc;
//         }
//     }

//     panic!("failed create");
// }

// #[no_mangle]
// pub unsafe extern "C" fn switch_context(prev_sp: *const i32, next_sp: *const i32) {
//         // loop {}
//         asm!(
//             // "addi sp, sp, -13 * 4",
//             // "sw ra,  0  * 4(sp)",
//             // "sw s0,  1  * 4(sp)",
//             // "sw s1,  2  * 4(sp)",
//             // "sw s2,  3  * 4(sp)",
//             // "sw s3,  4  * 4(sp)",
//             // "sw s4,  5  * 4(sp)",
//             // "sw s5,  6  * 4(sp)",
//             // "sw s6,  7  * 4(sp)",
//             // "sw s7,  8  * 4(sp)",
//             // "sw s8,  9  * 4(sp)",
//             // "sw s9,  10 * 4(sp)",
//             // "sw s10, 11 * 4(sp)",
//             // "sw s11, 12 * 4(sp)",
//             // "sw sp, 0(a0)",
//             "mv sp, a1",
//             // "lw ra,  0  * 4(sp)",
//             // "lw s0,  1  * 4(sp)",
//             // "lw s1,  2  * 4(sp)",
//             // "lw s2,  3  * 4(sp)",
//             // "lw s3,  4  * 4(sp)",
//             // "lw s4,  5  * 4(sp)",
//             // "lw s5,  6  * 4(sp)",
//             // "lw s6,  7  * 4(sp)",
//             // "lw s7,  8  * 4(sp)",
//             // "lw s8,  9  * 4(sp)",
//             // "lw s9,  10 * 4(sp)",
//             // "lw s10, 11 * 4(sp)",
//             // "lw s11, 12 * 4(sp)",
//             // "addi sp, sp, 13 * 4",
//             // "ret",
//             in("a0") prev_sp,
//             in("a1") next_sp,
//         );
// }
