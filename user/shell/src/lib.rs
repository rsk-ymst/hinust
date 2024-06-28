#![no_std]

use common::strcmp2;
use core::panic::PanicInfo;
use mystd::sys::*;
use mystd::*;
// use mystd::common::strcmp2;

#[no_mangle]
pub unsafe fn main() {
    println!("hello, shell");

    loop {
        let mut chars: [char; 256] = ['\0'; 256];
        let mut idx: usize = 0;

        print!("> ");

        /* 一文字ずつ入力を受けつける */
        loop {
            let ch = getchar();
            putchar(ch);

            /* Enterを押されたとき */
            if ch == '\r' {
                println!();
                break;
            }

            chars[idx] = ch;
            idx += 1;
        }

        // let target = ['h', 'e', 'l', 'l', 'o', '\0'].as_ptr();

        let target = ['e', 'x', 'i', 't', '\0'].as_ptr();
        if strcmp2(chars.as_ptr(), target as *mut char) {
            println!("exit!");
            exit_usr();
        }

        println!("unknown command: ");
    }

    // putchar('A');
    syscall(0, 0, 0, 0);
    loop {}
}

#[panic_handler]
#[no_mangle]
pub fn dummy_panic_shell(info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    // *(0x80200000) = 0x1234;
    loop {}
}
