// 標準ライブラリは使用しない
#![no_std]


use core::arch::{asm, global_asm};
use core::cell::Cell;
use core::fmt;
mod io;
mod kernel;
mod macros;
mod mem;
mod proc;

const PAGE_SIZE: usize = 4096;

use kernel::kernel_entry;

extern "C" {
    static mut __stack_top: *mut u8;
    // static mut kernel_entry: *mut u8;
}


#[derive(Debug)]
struct hoge {
    a: u8,
    b: u8,
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    print!("hoge");
    println!("hoge: {}", 0);
    unsafe {
        let hoge = hoge {
            a: 0, b: 0,
        };
        println!("hoge: {:?}", hoge);
    }

    unsafe {
        println!("pre: {}", __stack_top as i32);
    }

    let addr = fetch_address!("__free_ram");
    let addr2 = fetch_address!("__free_ram_end");

    println!("{:x}", addr);
    println!("{:x}", addr2);

    write_csr!("stvec", kernel_entry);

    unsafe {
        let mem_manager = PageManager {
            ram: RAM {
                entry_point: fetch_address!("__free_ram"),
                end_point: fetch_address!("__free_ram_end")
            },
            next_addr: Cell::new(fetch_address!("__free_ram")),
        };

        let paddr0: paddr_t = mem_manager.alloc_pages(2);
        let paddr1: paddr_t = mem_manager.alloc_pages(1);

        println!("alloc_pages test: paddr0={:x}", paddr0);
        println!("{:x}", *mem_manager.next_addr.as_ptr());

        println!("alloc_pages test: paddr1={:x}", paddr1);
        println!("{:x}", *mem_manager.next_addr.as_ptr());
        // println!("alloc_pages test: paddr1={:?}", paddr1);
    }

    // unsafe {
    //     asm!(
    //         "unimp",
    //     );
    // }


    // write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
    // unsafe {
    //     putchar(__stack_top as char);
    // }
    // }

    // UART0の送信バッファに1文字ずつデータをストアする
    // let uart0 = 0x10013000  as *mut u8;
    // for c in b"Hello from Rust!".iter() {
    //     unsafe {
    //         *uart0 = *c as u8;
    //     }
    // }

    loop {}
}

#[link_section = ".text.boot"]
#[no_mangle]
pub extern "C" fn boot() {
    unsafe {
        // __stack_top = 0x80200000 as *mut u8;
        asm!(
            "lui sp, %hi(__stack_top)",
            "addi sp, sp, %lo(__stack_top)",
            "j kernel_main",
            options(nostack)
        );
    }
}

#[no_mangle]
pub fn sbi_call(mut arg0: i32, mut arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, fid: i32, eid: i32) {
    /*
    　inoutは引数として使われ、かつ値が変わるもの
      inは単なる引数として使われる
      outは結果を書き込むものとして使われる
     */

    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0, // キャストっぽく認識すると良い
            inout("a1") arg1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            //            // out("a1") a1,
            // : a0 = inout(reg) arg0, a1 = inout(reg) a2
            // :
            clobber_abi("C"),
        );
    }
}

// #[no_mangle]
// fn putchar(ch: char) {
//     sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1 /* Console Putchar */);
// }

// panic発生時のハンドラ
use core::panic::PanicInfo;
use core::ptr::write_bytes;

use io::putchar;

use crate::mem::{paddr_t, PageManager, RAM};

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    loop{}
}

// abort時のハンドラ
#[no_mangle]
pub fn abort() -> ! {
    // 何もせず、無限ループする
    loop {}
}
