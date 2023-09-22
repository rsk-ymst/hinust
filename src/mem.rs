use lazy_static::lazy_static;

use crate::{fetch_address, println};
pub use core::arch::asm;
use core::cell::Cell;

const PAGE_SIZE: i32 = 4096;
pub type paddr_t = i32;



#[derive(Debug)]
pub struct RAM {
    pub entry_point: paddr_t,
    pub end_point: paddr_t,
}

impl RAM {
    pub fn is_valid_address(&self, paddr: paddr_t) -> bool {
        self.entry_point >= paddr && self.end_point <= paddr
    }
}

#[derive(Debug)]
pub struct PageManager {
    pub ram: RAM,
    pub next_addr: Cell<paddr_t>
}

impl PageManager {
    pub unsafe fn alloc_pages(&self, n: i32) -> paddr_t {
        let paddr: paddr_t = *self.next_addr.as_ptr();
        *self.next_addr.as_ptr() += n * PAGE_SIZE;

        if self.ram.is_valid_address(paddr) {
            println!("out of memory...");
        }

        self.alloc_zero(paddr as *mut u8, (n* PAGE_SIZE) as usize);
        paddr
    }

    pub fn alloc_zero(&self, buf: *mut u8, n: usize) -> *mut u8 {
        unsafe {
            buf.write_bytes(0, n);
            buf
        }
    }
}
