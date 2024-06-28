use core::arch::asm;
use common::sys::{SYS_GETCHAR, SYS_PUTCHAR};

#[no_mangle]
pub unsafe fn syscall(mut sysno: i32, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
    asm!(
        "ecall",
        inout("a0") arg0, // キャストっぽく認識すると良い
        in("a1") arg1,
        in("a2") arg2,
        in("a3") sysno,
    );

    arg0
}

pub unsafe fn putchar(c: char) {
    syscall(SYS_PUTCHAR, c as i32, 0, 0);
}

pub unsafe fn getchar() -> char {
    return (syscall(SYS_GETCHAR, 0, 0, 0) as u8) as char;
}
