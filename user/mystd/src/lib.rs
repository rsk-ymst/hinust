#![no_std]

use core::{arch::asm, fmt};
pub const SYS_PUTCHAR: i32 = 1;

pub struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_byte(s);
        Ok(())
    }
}

impl Writer {
    pub fn write_byte(&mut self, str: &str) {
        for c in str.chars() {
            putchar(c)
        }
    }
}

#[link_section = ".text.start"]
#[no_mangle]
pub extern "C" fn start() {
    unsafe {
        asm!(
            "lui sp, %hi(__stack_top)",
            "addi sp, sp, %lo(__stack_top)",
            "call main",
            "call exit",
            options(nostack)
        );
    }
}

#[no_mangle]
pub extern "C" fn exit() -> ! {
    loop {}
}

#[no_mangle]
pub fn syscall(mut sysno: i32, mut arg0: i32, mut arg1: i32, mut arg2: i32) -> i32 {
    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0, // キャストっぽく認識すると良い
            inout("a1") arg1,
            inout("a2") arg2,
            inout("a3") sysno,
        );
    }

    arg0
}

pub fn putchar(c: char) {
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

