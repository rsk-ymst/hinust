// 標準ライブラリは使用しない
#![no_std]


use core::arch::{asm, global_asm};
use core::borrow::BorrowMut;
use core::cell::Cell;
use core::fmt;
mod io;
mod kernel;
mod macros;
mod mem;
// mod proc;
mod proc2;

const PAGE_SIZE: usize = 4096;

use lazy_static::lazy_static;

use kernel::kernel_entry;
use proc2::{Process, State};

// lazy_static! {
//     static ref GLOBAL_POINTER: *mut i32 = x;
// }

static mut procs: [Process; 8] = [Process {
    pid: -1,
    state: State::PROC_UNUSED,
    sp: 0 as *mut i32,
    stack: [0; 8192]
}; 8];

static mut proc_a: Process = Process {
    pid: -1,
    state: State::PROC_RUNNABLE,
    sp: 0 as *mut i32,
    stack: [0; 8192]
};

static mut proc_b: Process = Process {
    pid: -1,
    state: State::PROC_RUNNABLE,
    sp: 0 as *mut i32,
    stack: [0; 8192]
};

extern "C" {
    static mut __stack_top: *mut u8;
    // static mut kernel_entry: *mut u8;

    #[allow(improper_ctypes)]
    fn switch_context(prev_sp: &*mut i32, next_sp: &*mut i32);
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

    // write_csr!("stvec", kernel_entry);
    // kernel_entry();

    unsafe {
        let mem_manager = PageManager {
            ram: RAM {
                entry_point: fetch_address!("__free_ram"),
                end_point: fetch_address!("__free_ram_end")
            },
            next_addr: Cell::new(fetch_address!("__free_ram")),
        };

        let bcc_size = fetch_address!("__bss_end") - fetch_address!("__bss");
        mem_manager.alloc_zero(fetch_address!("__bss") as *mut u8, bcc_size);
        // let mut proc_manager = ProcessManager::init();
        // let p = Process::init();

        // proc_a = *proc_manager.create_process(fetch_address!("proc_a_entry"));
        // proc_b = *proc_manager.create_process(fetch_address!("proc_b_entry"));

        proc_a = create_process(fetch_address!("proc_a_entry"));
        proc_b = create_process(fetch_address!("proc_b_entry"));

        // println!("{:?}", proc_a);
        // println!("{:?}", proc_b);

        proc_a_entry();
        // proc_b_entry();

        // let paddr0: paddr_t = mem_manager.alloc_pages(2);
        // let paddr1: paddr_t = mem_manager.alloc_pages(1);

        // println!("alloc_pages test: paddr0={:x}", paddr0);
        // println!("{:?}", *proc_manager.proc_a);

        // println!("alloc_pages test: paddr1={:x}", paddr1);
        // println!("{:x}", *mem_manager.next_addr.as_ptr());
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

#[no_mangle]
pub unsafe extern "C" fn proc_a_entry() {
    for _ in 0..30000000 {
        println!("A");
        // println!("{:x}, {:x}", *proc_a.sp, *proc_b.sp);
        switch_context(&proc_a.sp, &proc_b.sp);
        unsafe {
            asm!("nop");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn proc_b_entry() {
    for _ in 0..30000000 {
        println!("B");
        switch_context(&proc_b.sp, &proc_a.sp);

        unsafe {
            asm!("nop");
        }
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
            // clobber_abi("C"),
        );
    }
}

pub unsafe fn create_process(pc: i32) -> Process {
    for (i, proc) in procs.iter_mut().enumerate() {
        if proc.state == State::PROC_UNUSED {
            /* 全て0で初期化されているので、0でwriteする必要がない */
            let sp: *mut i32 = proc.stack.as_mut_ptr().add(proc.stack.len()).cast::<i32>();
            let top_sp = sp.offset(-12);

            top_sp.write(pc);

            proc.pid = (i + 1) as i32;
            proc.state = State::PROC_RUNNABLE;
            proc.sp = top_sp;

            return *proc;
        }
    }

    panic!("failed create");
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
// use crate::proc2::switch_context;
// use crate::proc2::switch_context;
// use crate::proc::{ProcessManager, Process, switch_context};
// use crate::proc2::create_process;

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
