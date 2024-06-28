#![no_std]

use core::{panic::PanicInfo};
use mystd::*;
use common::strcmp2;
use mystd::sys::*;

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
                break 1;
            }

            chars[idx] = ch;
            idx += 1;
        };


            let target = ['h', 'e', 'l', 'l', 'o', '\0'].as_ptr();

           if strcmp2(chars.as_ptr(), target as *mut char) {
                println!("yes, hello!");
           }
        
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
