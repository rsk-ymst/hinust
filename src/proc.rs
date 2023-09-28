use core::cell::{Cell, RefCell};
use core::{arch::asm, borrow::BorrowMut};


use crate::mem::{PageManager, PAGE_R, PAGE_W, PAGE_X, SATP_SV32, PAGE_SIZE};
use crate::{io::putchar, println};
use crate::{switch_context, fetch_address};


type vaddr_t = i32;

const PROC_MAX: usize = 3;


#[derive(Debug)]
#[repr(C, align(32))]
pub struct ProcessManager {
    pub procs: [Process; PROC_MAX],
    pub current_proc_idx: usize,
    pub idle_proc_idx: usize,
}

#[derive(Clone, Debug)]
#[repr(C, align(32))]
pub struct Process {
    pub pid: isize,         // プロセスID
    pub state: ProcState,   // プロセスの状態
    pub sp: usize,          // コンテキストスイッチ時のスタックポインタ
    pub page_table: usize,
    pub stack: [u8; 8192]
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProcState {
    RUNNABLE,
    UNUSED,
    STABLE,
}

pub static mut PROC_MANAGER: ProcessManager = ProcessManager {
    procs: [
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
            page_table: 0,
            },
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
            page_table: 0,
        },
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
            page_table: 0,
        },
    ],
    current_proc_idx: 0,
    idle_proc_idx: 0
};

impl ProcessManager {
    #[no_mangle]
    pub unsafe fn create_process(&mut self, pc: i32, mem_manager: &mut PageManager) {
        for (i, proc) in self.procs.iter_mut().enumerate() {
            if proc.state == ProcState::UNUSED {
                let sp = proc.stack.as_mut_ptr().add(proc.stack.len()).cast::<i32>();
                let top_sp = sp.offset(-12);

                top_sp.write(pc);

                let page_table = mem_manager.alloc_pages(1); // ページテーブルのサイズは4KB

                let __kernel_base = fetch_address!("__kernel_base");
                let __free_ram_end = fetch_address!("__free_ram_end");

                let mut paddr = __kernel_base;

                // mem_manager.alloc_pages(1);
                // mem_manager.alloc_pages(1);
                // mem_manager.alloc_pages(1);


                while paddr < __free_ram_end {
                    // println!("paddr!: {paddr:x}");
                    // あたかもカーネルが使える空間全体をサーバ側が使えるようになっている
                    mem_manager.map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
                    paddr += PAGE_SIZE;
                }

                proc.pid = (i + 1) as isize;
                proc.state = ProcState::RUNNABLE;
                proc.sp = top_sp as usize;
                proc.page_table = page_table;

                return;
            }
        }

        panic!("failed create");
    }

    #[no_mangle]
    pub unsafe extern "C" fn yield_(&mut self) {
        let mut next = self.idle_proc_idx;

        for i in 0..PROC_MAX {
            let proc = &self.procs[i];

            if i == self.current_proc_idx {
                continue;
            }

            if proc.state == ProcState::RUNNABLE && proc.pid > 0 {
                next = i;
                break;
            }
        }

        if next == self.current_proc_idx {
            println!("ret");
            return;
        }

        // asm!(
        //     "csrw sscratch, {0}", // sscratchは重要な情報の退避用レジスタ
        //     in(reg) self.procs[next].stack[8191] as *const u8
        // );

        // println!("page_table --> {:x}", self.procs[next].page_table);
        // println!("page_table_idx --> {}", (self.procs[next].page_table / PAGE_SIZE) & 0x3ff);
        // println!("...{:b}", SATP_SV32);


        asm!(
            "sfence.vma",
            "csrw satp, {0}",
            "sfence.vma",
            "csrw sscratch, {1}", // sscratchは重要な情報の退避用レジスタ
            in(reg) (SATP_SV32 | self.procs[next].page_table / PAGE_SIZE) as isize,
            in(reg) self.procs[next].stack[8191] as *const u8
        );

        let prev = self.current_proc_idx;
        self.current_proc_idx = next;

        switch_context(&self.procs[prev].sp, &self.procs[self.current_proc_idx].sp);
    }
}
