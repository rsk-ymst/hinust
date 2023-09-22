use core::cell::{Cell, RefCell};
use core::{arch::asm, borrow::BorrowMut};


use crate::proc2::ProcState;
use crate::{io::putchar, println};
use crate::{Process, switch_context};


type vaddr_t = i32;

const PROC_MAX: usize = 3;


#[derive(Debug)]
#[repr(C, align(32))]
pub struct ProcessManager {
    pub procs: [Process; PROC_MAX],
    pub current_proc_idx: usize,
    pub idle_proc_idx: usize,
}

pub static mut PROC_MANAGER: ProcessManager = ProcessManager {
    procs: [
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
            },
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
        },
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
        },
    ],
    current_proc_idx: 0,
    idle_proc_idx: 0
};

impl ProcessManager {
    #[no_mangle]
    pub unsafe fn create_process(&mut self, pc: i32) {
        for (i, proc) in self.procs.iter_mut().enumerate() {
            if proc.state == ProcState::UNUSED {
                let sp = proc.stack.as_mut_ptr().add(proc.stack.len()).cast::<i32>();
                let top_sp = sp.offset(-12);

                top_sp.write(pc);

                proc.pid = (i + 1) as isize;
                proc.state = ProcState::RUNNABLE;
                proc.sp = top_sp as usize;

                return;
            }
        }

        panic!("failed create");
    }

    #[no_mangle]
    pub unsafe extern "C" fn yield_(&mut self) {
        let mut next = self.idle_proc_idx;

        for i in 0..PROC_MAX {
            let proc = &self.procs[i];

            if i == self.current_proc_idx {
                continue;
            }

            if proc.state == ProcState::RUNNABLE && proc.pid > 0 {
                next = i;
                break;
            }
        }

        if next == self.current_proc_idx {
            println!("ret");
            return;
        }

        // println!("point: {:?}", &self.procs[next].stack[8191] as *const u8);
        asm!(
            "csrw sscratch, {0}", // sscratchは重要な情報の退避用レジスタ
            in(reg) self.procs[next].stack[8191] as *const u8
        );

        // &self.procs[self.current_proc_idx].stack[8192]

        let prev = self.current_proc_idx;
        self.current_proc_idx = next;

        // println!("point: {:?}", &self.procs[prev].sp);
        // println!("point２: {:?}", self.procs[prev].sp);

        switch_context(&self.procs[prev].sp, &self.procs[self.current_proc_idx].sp);
    }
}
