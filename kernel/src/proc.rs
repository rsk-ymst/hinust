use core::cell::{Cell, RefCell};
use core::ffi::c_void;
use core::ptr;
use core::{arch::asm, borrow::BorrowMut};

use crate::mem::SSTATUS_SPIE;
use crate::mem::{paddr_t, PageManager, PAGE_R, PAGE_SIZE, PAGE_U, PAGE_W, PAGE_X, SATP_SV32};
use crate::println;
use crate::{fetch_address, switch_context};

type vaddr_t = i32;

const PROC_MAX: usize = 4;

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
    pub pid: isize,       // プロセスID
    pub state: ProcState, // プロセスの状態
    pub sp: usize,        // コンテキストスイッチ時のスタックポインタ
    pub page_table: usize,
    pub stack: [u8; 8192],
}

#[repr(C, align(32))]
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
        Process {
            pid: -1,
            state: ProcState::UNUSED,
            sp: 0,
            stack: [0; 8192],
            page_table: 0,
        },
    ],
    current_proc_idx: 0,
    idle_proc_idx: 0,
};

#[no_mangle]
// #[naked]
unsafe fn user_entry() {
    asm!(
        "csrw sepc, {0}", // ユーザランドのプログラムカウンタを設定
        "csrw sstatus, {1}", // SPIEを立て，割り込みを有効か
        "sret",
        in(reg) USER_BASE,
        in (reg) SSTATUS_SPIE,
        options(noreturn)
    );
}

pub const USER_BASE: paddr_t = 0x1000000;

impl ProcessManager {
    #[no_mangle]
    pub unsafe fn create_process(
        &mut self,
        image: *mut c_void,
        image_size: i32,
        mem_manager: &mut PageManager,
    ) {
        for (i, proc) in self.procs.iter_mut().enumerate() {
            if proc.state != ProcState::UNUSED {
                continue;
            }

            let stack_bottom = proc.stack.as_mut_ptr().add(proc.stack.len()).cast::<i32>();
            let sp = stack_bottom.offset(-12);

            sp.write(user_entry as i32);

            // 各プロセスに対してページテーブル1段目を作成
            let page_table = mem_manager.alloc_pages(1); // ページテーブルのサイズは4KB

            let __kernel_base = fetch_address!("__kernel_base");
            let __free_ram_end = fetch_address!("__free_ram_end");

            let mut paddr = __kernel_base;

            // あたかもカーネル空間全体をユーザランド側が使えるようになっている
            // カーネルのページをマッピング
            while paddr < __free_ram_end {
                // 仮想アドレスと物理アドレスの対応付け
                mem_manager.map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
                paddr += PAGE_SIZE;
            }

            if image.is_null() {
                return;
            }

            // println!("image size {:x}", image_size);
            for offset in (0..image_size).step_by(PAGE_SIZE) {
                // println!(" --> {:x}", offset);
                // println!("offset --> {}", offset);
                let page = mem_manager.alloc_pages(1);

                // ユーザプログラムのバイナリ情報をコピー
                ptr::copy(
                    image.offset(offset as isize),
                    page as *mut c_void,
                    PAGE_SIZE,
                );

                // コピーしたバイナリの情報とページテーブルを対応付け
                mem_manager.map_page(
                    page_table,
                    USER_BASE + offset as usize,
                    page,
                    PAGE_U | PAGE_R | PAGE_W | PAGE_X,
                );
            }

            proc.pid = (i + 1) as isize;
            proc.state = ProcState::RUNNABLE;
            proc.sp = sp as usize;
            proc.page_table = page_table;

            return;
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
            "csrw satp, {}",
            "sfence.vma",
            "csrw sscratch, {}", // sscratchは重要な情報の退避用レジスタ
            in(reg) (SATP_SV32 | self.procs[next].page_table / PAGE_SIZE) as isize,
            in(reg) (&(self.procs[next].stack[8191]) as *const u8)// ここ適切に設定しないと割り込みが機能しない！！
        );

        let prev = self.current_proc_idx;
        self.current_proc_idx = next;

        switch_context(&self.procs[prev].sp, &self.procs[next].sp);
    }
}
