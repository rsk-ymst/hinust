// pub use crate::{cramp32_csrs, cramp32_csrsi};
pub use core::arch::asm;

#[macro_export]
macro_rules! cramp32_csrsi {
    ($reg: expr, $imm: expr $(,)?) => {
        if $imm < 32 {
            unsafe {
                asm!(concat!("csrsi ", $reg, ", ", $imm));
            }
        } else {
            unsafe {
                asm!(concat!("csrs ", $reg, ", {0}"), in(reg) $imm);
            }
        }
    }
}

#[macro_export]
macro_rules! read_csr {
    ($reg: expr) => {
        unsafe {
            let mut result = 0;
            asm!(concat!("csrr {0}, ", $reg), out(reg) result);
            result
        }
    }
}

#[macro_export]
macro_rules! write_csr {
    ($reg: expr, $val: expr) => {
        unsafe {
            asm!(concat!("csrw ", $reg, ", {0}"), in(reg) $val);
        }
    }
}

#[macro_export]
macro_rules! fetch_address {
    ($symbol:expr) => {
        unsafe {
            let mut result = 0;
            asm!(concat!("lla {0}, ", $symbol), out(reg) result);
            result
        }
    }
}
