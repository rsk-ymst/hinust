#![no_std]

use core::panic::PanicInfo;
use mystd::*;

#[no_mangle]
pub unsafe fn main() {
    println!("hello, shell");
    print!("> ");

    let ch = getchar();
    println!("ch: {}", ch as u8 as char);
    putchar(ch);

    // putchar('A');
    // syscall(0, 0, 0, 0);
    loop {}
}

#[panic_handler]
#[no_mangle]
pub fn dummy_panic_shell(info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    // *(0x80200000) = 0x1234;
    loop {}
}
