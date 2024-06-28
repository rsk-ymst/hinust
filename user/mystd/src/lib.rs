#![no_std]

use core::{arch::asm, fmt};
use crate::sys::putchar;

pub mod sys;
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

