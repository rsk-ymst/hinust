#![no_std]

use core::{arch::asm, fmt, panic::PanicInfo};

pub mod sys;

pub struct Writer {}

// lazy_static! {
//     pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {});
// }

// pub static WRITER: Writer = Writer {};

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_byte(s);
        Ok(())
    }
}

impl Writer {
    pub fn write_byte(&mut self, str: &str) {
        for c in str.chars() {
            unsafe { putchar(c) }
        }
    }
}

#[no_mangle]
pub unsafe fn putchar(ch: char) {
    sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
}

/* 以下は定型文 */
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// #[macro_export]
// macro_rules! println2 {
//     () => ($crate::print!("\n"));
//     ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
// }

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut WRITER = Writer {};
    WRITER.write_fmt(args).unwrap();
}

pub struct SbiRet {
    pub error: i32,
    pub value: i32,
}

#[no_mangle]
pub unsafe fn sbi_call(
    mut arg0: i32,
    mut arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    fid: i32,
    eid: i32,
) -> SbiRet {
    /*
        inoutは引数として使われ、かつ値が変わるもの
        inは単なる引数として使われる
        outは結果を書き込むものとして使われる
    */

    asm!(
        "ecall",
        inout("a0") arg0, 
        inout("a1") arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        in("a6") fid,
        in("a7") eid,

        clobber_abi("C"), 
    );

    SbiRet {
        error: arg0,
        value: arg1,
    }
}

// #[panic_handler]
// #[no_mangle]
// pub fn dummy_panic(info: &PanicInfo) -> ! {
//     // 何もせず、無限ループする
//     // println!("{}", info);
//     loop{}
// }
