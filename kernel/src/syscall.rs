use common::{println, putchar, sbi_call, sys::SYS_GETCHAR, sys::SYS_PUTCHAR};

use crate::{kernel::TrapFrame, proc::PROC_MANAGER};


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
        SYS_GETCHAR => {
            loop {
                let ch = getchar();

                if ch >= 0 {
                    (*frame).a0 = ch;
                    return;
                }

                PROC_MANAGER.yield_();
            }
        }
        _ => {
            panic!()
        }
    }
}
