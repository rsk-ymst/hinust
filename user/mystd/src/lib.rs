#![no_std]

use core::{arch::asm, fmt};
pub const SYS_PUTCHAR: i32 = 1;

pub struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            self.write_byte(s);
        }
        Ok(())
    }
}

impl Writer {
    pub unsafe fn write_byte(&mut self, str: &str) {
        for c in str.chars() {
            putchar(c)
        }
    }
}

#[link_section = ".text.start"]
#[no_mangle]
pub unsafe extern "C" fn start() {
    asm!(
        "lui sp, %hi(__stack_top)",
        "addi sp, sp, %lo(__stack_top)",
        "call main",
        "call exit",
        options(nostack)
    );
}

#[no_mangle]
pub extern "C" fn exit() -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn syscall(mut sysno: i32, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
    asm!(
        "mv a0, {0}",
        "mv a1, {1}",
        "mv a2, {2}",
        "mv a3, {3}",
        "ecall",
        in(reg) arg0, // キャストっぽく認識すると良い
        in(reg) arg1,
        in(reg) arg2,
        in(reg) sysno,
    );

    arg0
}

pub unsafe fn putchar(c: char) {
    syscall(SYS_PUTCHAR, c as i32, 0, 0);
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut WRITER = Writer {};
    WRITER.write_fmt(args).unwrap();
}

/* 以下は定型文 */
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

/* 以下は定型文 */
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

