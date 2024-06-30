use common::{
    println, putchar, sbi_call,
    sys::{SYS_EXIT, SYS_GETCHAR, SYS_PUTCHAR},
};

use crate::{
    kernel::TrapFrame,
    proc::{ProcState, PROC_MANAGER},
};

pub unsafe fn getchar() -> i32 {
    let ret = sbi_call(0, 0, 0, 0, 0, 0, 0, 2);
    return ret.error as i32;
}

pub unsafe fn handle_syscall(trap_frame: *mut TrapFrame) {
    let frame = trap_frame;

    match (*frame).a3 {
        SYS_PUTCHAR => {
            putchar(((*frame).a0 as u8) as char);
            return;
        }
        SYS_GETCHAR => loop {
            let ch = getchar();

            if ch >= 0 {
                (*frame).a0 = ch;
                return;
            }

            PROC_MANAGER.yield_();
        },
        SYS_EXIT => {
            println!("exit syscall: {}", PROC_MANAGER.current_proc_idx);

            PROC_MANAGER.procs[PROC_MANAGER.current_proc_idx].state = ProcState::UNUSED;
            PROC_MANAGER.yield_();

            panic!("exit successfully");
        }

        _ => {
            panic!()
        }
    }
}
