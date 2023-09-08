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
            pid: Cell::new(-1),
            state: Cell::new(ProcState::UNUSED),
            sp: Cell::new(0 as *mut i32),
            stack: Cell::new([0; 8192]),
            },
        Process {
            pid: Cell::new(-1),
            state: Cell::new(ProcState::UNUSED),
            sp: Cell::new(0 as *mut i32),
            stack: Cell::new([0; 8192]),
            },
        Process {
            pid: Cell::new(-1),
            state: Cell::new(ProcState::UNUSED),
            sp: Cell::new(0 as *mut i32),
            stack: Cell::new([0; 8192]),
            },
        ],
    current_proc_idx: 0,
    idle_proc_idx: 0
    // proc_a: todo!(),
    // proc_b: todo!(),
};

impl ProcessManager {
    #[no_mangle]
    pub unsafe fn create_process(&mut self, pc: i32) {
        for (i, proc) in self.procs.iter_mut().enumerate() {
            if proc.state.get() == ProcState::UNUSED {
                /* 全て0で初期化されているので、0でwriteする必要がない */
                let sp = (*(proc.stack.get_mut())).as_mut_ptr().add(proc.stack.get().len()).cast::<i32>();
                let top_sp = sp.offset(-12);

                top_sp.write(pc);

                *proc.pid.get_mut() = (i + 1) as i32;
                *proc.state.get_mut() = ProcState::RUNNABLE;
                *proc.sp.get_mut() = top_sp;

                // proc.replace(
                // Process {
                //         pid: (i + 1) as i32,
                //         state: ProcState::RUNNABLE,
                //         sp: top_sp,
                //         stack: proc.read().stack,
                //     }
                // );

                return;
            }
        }

        panic!("failed create");
    }

    #[no_mangle]
    pub unsafe extern "C" fn yield_(&mut self) {
        // println!("yield!: cur: {:?}", self.current_proc_idx.as_ptr().read().pid);
        let mut next = self.idle_proc_idx;
        // println!("yield!: cur:");

        for i in 0..PROC_MAX {
            // println!("yield!: proc: {} ", self.current_proc_idx.as_ptr().read().pid);
            // let pid = self.current_proc_idx.as_ptr().read().pid;
            // println!("idx: {}", (pid + i));
            // let index = (pid + i);

            // println!("yield!: proc: {} ", (self.current_proc_idx.as_ptr().read().pid as usize + i) % PROC_MAX);
            let proc = &self.procs[i];
            println!("yield!: {} pid: {:?}", i, proc.pid.get());

            // let x = self.current_proc_idx; // これが怪しそう
            if i == self.current_proc_idx {
                println!("----------- yield!: cur_idx: {} ", self.current_proc_idx);

                continue;
            }
            println!("yield!: state: {:?}", proc.state);
            if proc.state.get() == ProcState::RUNNABLE && proc.pid.get() > 0 {
                // println!("yield!: hgoe: {:?}", proc.pid);
                next = i;
                break;
            }
            // i += 1;
            // println!("yield!: i: {:?}", i);
        }

        // println!("yield!: cur: {:?}", self.current_proc_idx.as_ptr().read().pid);
        // println!("yield!: next2: {:?}", next.read().pid);
        if next == self.current_proc_idx {
            println!("ret");
            return;
        }

        let prev = self.current_proc_idx;
        self.current_proc_idx = next;

        // self.procs[self.current_proc_idx].state = ProcState::RUNNABLE;
        println!("yield!: next: {:?}", next);

        switch_context(&self.procs[prev].sp.get(), &self.procs[self.current_proc_idx].sp.get());
    }
}
