use lazy_static::lazy_static;

use crate::{fetch_address, println, utils::is_aligned};
pub use core::arch::asm;
use core::cell::Cell;

pub const PAGE_SIZE: usize = 4096;
pub type paddr_t = usize;

pub const SATP_SV32: usize = 1 << 31;
pub const PAGE_V: usize = 1 << 0; // 有効化どうか
pub const PAGE_R: usize = 1 << 1;
pub const PAGE_W: usize = 1 << 2;
pub const PAGE_X: usize = 1 << 3;
pub const PAGE_U: usize = 1 << 4;

#[derive(Debug, Clone)]
pub struct RAM {
    pub entry_point: paddr_t,
    pub end_point: paddr_t,
}

impl RAM {
    pub fn is_valid_address(&self, paddr: paddr_t) -> bool {
        self.entry_point >= paddr && self.end_point <= paddr
    }
}

#[derive(Debug, Clone)]
pub struct PageManager {
    pub ram: RAM,
    pub next_addr: Cell<paddr_t>
}

// static mut MEM_MANAGER: PageManager = PageManager {
//     ram: RAM {
//         entry_point: fetch_address!("__free_ram"),
//         end_point: fetch_address!("__free_ram_end")
//     },
//     next_addr: Cell::new(fetch_address!("__free_ram")),
// };

impl PageManager {
    pub unsafe fn alloc_pages(&self, n: usize) -> paddr_t {
        let paddr: paddr_t = *self.next_addr.as_ptr();
        self.next_addr.set(paddr + n * PAGE_SIZE);

        if self.ram.is_valid_address(paddr) {
            println!("out of memory...");
        }

        self.alloc_zero(paddr as *mut u8, (n* PAGE_SIZE ) as usize);
        paddr
    }

    pub fn alloc_zero(&self, buf: *mut u8, n: usize) -> *mut u8 {
        unsafe {
            buf.write_bytes(0, n);
            buf
        }
    }

    pub unsafe fn map_page(&mut self, table1: usize, vaddr: usize, paddr: usize, flags: usize) {
        if is_aligned(vaddr, PAGE_SIZE) {
            panic!();
        }

        if is_aligned(paddr, PAGE_SIZE) {
            panic!();
        }

        let vpn1 = (vaddr >> 22) & 0x3ff;
        let pt0_paddr = (table1 as *mut usize).offset(vpn1 as isize);

        if *pt0_paddr & PAGE_V == 0 {
            // 2段目のページテーブルが存在しないので作成する
            let pt_paddr = unsafe { self.alloc_pages(1) };

            // ページ単位でどこにあるかを格納
            pt0_paddr.write(((pt_paddr / PAGE_SIZE) << 10) | PAGE_V);
        }

        // 2段目のページテーブルにエントリを追加する
        let vpn0: usize = (vaddr >> 12) & 0x3ff;
        let table0 = ((*pt0_paddr >> 10) * PAGE_SIZE) as *mut usize; // アドレス値に変換

        table0.offset(vpn0 as isize).write(((paddr / PAGE_SIZE) << 10) | flags |PAGE_V);
    }
}
