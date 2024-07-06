#![no_std]

use common::strcmp2;
use core::panic::PanicInfo;
use core::str::Chars;
use mystd::sys::*;
use mystd::*;

#[no_mangle]
pub unsafe fn main() {
    println!("hello, shell");

    loop {
        let mut input: [char; 256] = ['\0'; 256];
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

            input[idx] = ch;
            idx += 1;
        }

        // let x = "hogehoge";

        let input = &input[0..idx];

        // input[0..idx].eq(&"exit".chars().into_iter());

        // "exit".char_indices().eq(x.chars().enumerate());
        
        if strcmp("exit", input) {
            println!("exit!");
            exit_usr();
        }

        // let target = ['e', 'x', 'i', 't', '\0'].as_ptr();
        // if strcmp2(chars.as_ptr(),  as *mut char) {
        //     println!("exit!");
        //     exit_usr();
        // }

        println!("unknown command: ");
    }

    // putchar('A');
    syscall(0, 0, 0, 0);
    loop {}
}

pub fn strcmp(chars: &str, target: &[char]) -> bool {
    for (i, c) in chars.char_indices() {
        if c != target[i] {
            return false;
        }
    }

    true
}

#[panic_handler]
#[no_mangle]
pub fn dummy_panic_shell(info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    // *(0x80200000) = 0x1234;
    loop {}
}
