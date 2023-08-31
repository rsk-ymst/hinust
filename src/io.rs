use core::fmt;

use spin::Mutex;
use lazy_static::lazy_static;

use crate::sbi_call;

pub struct Writer {}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {});
}

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

#[no_mangle]
pub fn putchar(ch: char) {
    sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}