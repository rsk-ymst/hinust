// 標準ライブラリは使用しない
#![no_std]


use core::{arch::asm, cell::Cell};
mod io;
mod kernel;
mod macros;
mod mem;
mod proc;
mod utils;
// mod use

const PAGE_SIZE: usize = 4096;

use lazy_static::lazy_static;

use kernel::kernel_entry;
use proc::{PROC_MANAGER, ProcessManager, Process, ProcState};

extern "C" {
    static mut __stack_top: *mut u8;
    static mut __kernel_base: paddr_t;
    static mut __free_ram: paddr_t;
    static mut __free_ram_end: paddr_t;
    // static mut kernel_entry: *mut u8;

    // #[allow(improper_ctypes)]
    fn switch_context(prev_sp: &usize, next_sp: &usize);
}


#[derive(Debug)]
struct hoge {
    a: u8,
    b: u8,
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    write_csr!("stvec", kernel_entry);

        let mut MEM_MANAGER: PageManager = PageManager {
            ram: RAM {
                entry_point: fetch_address!("__free_ram"),
                end_point: fetch_address!("__free_ram_end")
            },
            next_addr: Cell::new(fetch_address!("__free_ram")),
        };

        // println!("hello, world!");

        PROC_MANAGER.create_process(0, &mut MEM_MANAGER);

        PROC_MANAGER.procs[0].pid = -1;
        PROC_MANAGER.idle_proc_idx = 0;
        PROC_MANAGER.current_proc_idx = PROC_MANAGER.idle_proc_idx;

        PROC_MANAGER.create_process(fetch_address!("proc_a_entry_v2"), &mut MEM_MANAGER);
        PROC_MANAGER.create_process(fetch_address!("proc_b_entry_v2"), &mut MEM_MANAGER);
        PROC_MANAGER.create_process(fetch_address!("proc_c_entry_v2"), &mut MEM_MANAGER);

        // proc_a_entry_v2();

        PROC_MANAGER.yield_();

    loop {}
}

// #[no_mangle]
// pub unsafe extern "C" fn proc_a_entry() {
//     for _ in 0..30000000 {
//         println!("A");
//         // println!("{:x}, {:x}", *proc_a.sp, *proc_b.sp);
//         // switch_context(&proc_a.sp, &proc_b.sp);
//         // yield_();

//         unsafe {
//             asm!("nop");
//         }
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn proc_b_entry() {
//     for _ in 0..300_000_000 {
//         println!("B");
//         // switch_context(&proc_b.sp, &proc_a.sp);
//         // yield_();


//         unsafe {
//             asm!("nop");
//         }
//     }
// }

#[no_mangle]
pub unsafe extern "C" fn proc_a_entry_v2() {
    for _ in 0..300_000_000 {
        // #[cfg(debug_assertions)]
        println!("AAA");

        for _ in 0..1_000_000 {
            unsafe {
                asm!("nop");
            }
        }

        PROC_MANAGER.yield_();
    }
}

#[no_mangle]
pub unsafe extern "C" fn proc_b_entry_v2() {
    for _ in 0..30000000 {
        // #[cfg(debug_assertions)]
        println!("B");
        // switch_context(&PROC_MANAGER.procs[2].sp, &PROC_MANAGER.procs[1].sp);

        for _ in 0..1_000_000 {
            unsafe {
                asm!("nop");
            }
        }

        PROC_MANAGER.yield_();
    }
}

#[no_mangle]
pub unsafe extern "C" fn proc_c_entry_v2() {
    for _ in 0..30000000 {
        // #[cfg(debug_assertions)]
        println!("C");
        // switch_context(&PROC_MANAGER.procs[2].sp, &PROC_MANAGER.procs[1].sp);

        for _ in 0..1_000_000 {
            unsafe {
                asm!("nop");
            }
        }

        PROC_MANAGER.yield_();
    }
}

#[no_mangle]
pub extern "C" fn hello() {
    loop {
        println!("A");

        for i in 0..30000000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}

#[link_section = ".text.boot"]
// #[male]
#[no_mangle]
pub extern "C" fn boot() {
    unsafe {
        // __stack_top = 0x80200000 as *mut u8;
        asm!(
            "la sp, __stack_top",
            // "addi sp, sp, %lo(__stack_top)",
            "j kernel_main",
            options(noreturn, raw),
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
            // inout("a0") arg0, // キャストっぽく認識すると良い
            // inout("a1") arg1,
            // in("a2") arg2,
            // in("a3") arg3,
            // in("a4") arg4,
            // in("a5") arg5,
            // in("a6") fid,
            // in("a7") eid,
            //            // out("a1") a1,
            // : a0 = inout(reg) arg0, a1 = inout(reg) a2
            // :
            clobber_abi("C"), //
        );
    }
}

// panic発生時のハンドラ
use core::panic::PanicInfo;
use core::ptr::write_bytes;

use io::putchar;

use crate::mem::{paddr_t, PageManager, RAM};


#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    println!("{}", info);
    loop{}
}

// abort時のハンドラ
#[no_mangle]
pub fn abort() -> ! {
    // 何もせず、無限ループする
    loop {}
}
