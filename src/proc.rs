use core::{arch::asm, borrow::BorrowMut};

use crate::{io::putchar, println};


type vaddr_t = i32;

const PROC_MAX: usize = 8;


#[derive(Debug)]
pub struct ProcessManager {
    pub procs: [Process; PROC_MAX],
    pub proc_a: *mut Process,
    pub proc_b: *mut Process,
}

impl ProcessManager {
    pub fn init() -> Self {

        Self {
            procs: [Process::init(); PROC_MAX],
            proc_a: Process::init().borrow_mut(),
            proc_b: Process::init().borrow_mut(),
        }
    }

    pub unsafe fn create_process(&mut self, pc: i32) -> *mut Process {
        let mut proc = Process::init();

        for (i, proc) in self.procs.iter_mut().enumerate() {
            if proc.state == State::PROC_UNUSED {
                let sp: *mut i32 = proc.stack as *mut i32;

                for i in 0..13 {
                    *sp.offset(-i) = 0;
                    if i == 12 {
                        *sp.offset(-i) = pc;
                    }
                }

                proc.pid = (i + 1) as i32;
                proc.state = State::PROC_RUNNABLE;
                proc.sp = unsafe { *sp };

                return proc;
            }
        }

        panic!("failed create");
    }

    pub unsafe fn proc_a_entry(&self) {
        for _ in 0..30000000 {
            println!("A");
            switch_context(self.proc_a.read().sp as *const i32, self.proc_b.read().sp as *const i32);
            unsafe {
                asm!("nop");
            }
        }
    }

    pub unsafe extern "C"  fn proc_b_entry(&self) {
        for _ in 0..30000000 {
            println!("B");
            switch_context(self.proc_a.read().sp as *const i32, self.proc_b.read().sp as *const i32);
            unsafe {
                asm!("nop");
            }
        }
    }


}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    PROC_RUNNABLE,
    PROC_UNUSED,
    PROC_STABLE,
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pid: i32,      // プロセスID
    state: State,          // プロセスの状態
    sp: vaddr_t,   // コンテキストスイッチ時のスタックポインタ
    stack: *mut [u8; 8192]
}

impl Process {
    pub fn init() -> Self {
        println!("yes!");
        Self {
            pid: -1,                 // プロセスID
            state: State::PROC_UNUSED, // プロセスの状態
            sp: -1,                  // コンテキストスイッチ時のスタックポインタ
            stack: [0; 8192].borrow_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn switch_context(prev_sp: *const i32, next_sp: *const i32) {
     unsafe {
        asm!(
            "addi sp, sp, -13 * 4",
            "sw ra,  0  * 4(sp)",
            "sw s0,  1  * 4(sp)",
            "sw s1,  2  * 4(sp)",
            "sw s2,  3  * 4(sp)",
            "sw s3,  4  * 4(sp)",
            "sw s4,  5  * 4(sp)",
            "sw s5,  6  * 4(sp)",
            "sw s6,  7  * 4(sp)",
            "sw s7,  8  * 4(sp)",
            "sw s8,  9  * 4(sp)",
            "sw s9,  10 * 4(sp)",
            "sw s10, 11 * 4(sp)",
            "sw s11, 12 * 4(sp)",
            "sw sp, (a0)",
            "lw sp, (a1)",
            "lw ra,  0  * 4(sp)",
            "lw s0,  1  * 4(sp)",
            "lw s1,  2  * 4(sp)",
            "lw s2,  3  * 4(sp)",
            "lw s3,  4  * 4(sp)",
            "lw s4,  5  * 4(sp)",
            "lw s5,  6  * 4(sp)",
            "lw s6,  7  * 4(sp)",
            "lw s7,  8  * 4(sp)",
            "lw s8,  9  * 4(sp)",
            "lw s9,  10 * 4(sp)",
            "lw s10, 11 * 4(sp)",
            "lw s11, 12 * 4(sp)",
            "addi sp, sp, 13 * 4",
            "ret",
            in("a0") prev_sp,
            in("a1") next_sp,
        );
    }
}
