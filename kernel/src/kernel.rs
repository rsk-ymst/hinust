use core::arch::asm;

use common::putchar;

use crate::{println, proc::PROC_MANAGER, read_csr, utils::is_aligned, write_csr, PAGE_SIZE};

pub const PAGE_V: isize = 1 << 0; // 有効化どうか
const PAGE_R: isize = 1 << 1;
const PAGE_W: isize = 1 << 2;
const PAGE_X: isize = 1 << 3;
const PAGE_U: isize = 1 << 4;

// pub fn map_page(table1: usize, vaddr: usize, paddr: usize, flags: usize) {
//     if is_aligned(vaddr, PAGE_SIZE) {
//         panic!();
//     }

//     if is_aligned(paddr, PAGE_SIZE) {
//         panic!();
//     }

//     let vpn1: usize = (vaddr >> 22) & 0x3ff;

//     if unsafe { *(table1 as *mut usize).offset(vpn1 as isize) } & PAGE_V == 0 {
//         // let pt_addr = ME
//     }
// }

#[no_mangle]
pub extern "C" fn kernel_entry() {
    /*
    inoutは引数として使われ、かつ値が変わるもの
      inは単なる引数として使われる
      outは結果を書き込むものとして使われる
     */

    unsafe {
        // __stack_top = 0x80200000 as *mut u8;
        asm!(
            "csrrw sp, sscratch, sp",
            "addi sp, sp, -4 * 31",
            "sw ra,  4 * 0(sp)",
            "sw gp,  4 * 1(sp)",
            "sw tp,  4 * 2(sp)",
            "sw t0,  4 * 3(sp)",
            "sw t1,  4 * 4(sp)",
            "sw t2,  4 * 5(sp)",
            "sw t3,  4 * 6(sp)",
            "sw t4,  4 * 7(sp)",
            "sw t5,  4 * 8(sp)",
            "sw t6,  4 * 9(sp)",
            "sw a0,  4 * 10(sp)",
            "sw a1,  4 * 11(sp)",
            "sw a2,  4 * 12(sp)",
            "sw a3,  4 * 13(sp)",
            "sw a4,  4 * 14(sp)",
            "sw a5,  4 * 15(sp)",
            "sw a6,  4 * 16(sp)",
            "sw a7,  4 * 17(sp)",
            "sw s0,  4 * 18(sp)",
            "sw s1,  4 * 19(sp)",
            "sw s2,  4 * 20(sp)",
            "sw s3,  4 * 21(sp)",
            "sw s4,  4 * 22(sp)",
            "sw s5,  4 * 23(sp)",
            "sw s6,  4 * 24(sp)",
            "sw s7,  4 * 25(sp)",
            "sw s8,  4 * 26(sp)",
            "sw s9,  4 * 27(sp)",
            "sw s10, 4 * 28(sp)",
            "sw s11, 4 * 29(sp)",
            "csrr a0, sscratch",
            "sw a0, 4 * 30(sp)",
            "addi a0, sp, 4 * 31",
            "csrw sscratch, a0",
            "mv a0, sp",
            "call handle_trap",
            "lw ra,  4 * 0(sp)",
            "lw gp,  4 * 1(sp)",
            "lw tp,  4 * 2(sp)",
            "lw t0,  4 * 3(sp)",
            "lw t1,  4 * 4(sp)",
            "lw t2,  4 * 5(sp)",
            "lw t3,  4 * 6(sp)",
            "lw t4,  4 * 7(sp)",
            "lw t5,  4 * 8(sp)",
            "lw t6,  4 * 9(sp)",
            "lw a0,  4 * 10(sp)",
            "lw a1,  4 * 11(sp)",
            "lw a2,  4 * 12(sp)",
            "lw a3,  4 * 13(sp)",
            "lw a4,  4 * 14(sp)",
            "lw a5,  4 * 15(sp)",
            "lw a6,  4 * 16(sp)",
            "lw a7,  4 * 17(sp)",
            "lw s0,  4 * 18(sp)",
            "lw s1,  4 * 19(sp)",
            "lw s2,  4 * 20(sp)",
            "lw s3,  4 * 21(sp)",
            "lw s4,  4 * 22(sp)",
            "lw s5,  4 * 23(sp)",
            "lw s6,  4 * 24(sp)",
            "lw s7,  4 * 25(sp)",
            "lw s8,  4 * 26(sp)",
            "lw s9,  4 * 27(sp)",
            "lw s10, 4 * 28(sp)",
            "lw s11, 4 * 29(sp)",
            "lw sp,  4 * 30(sp)",
            "sret",
        );
    }
}

pub struct trap_frame {
    pub ra: i32,
    pub gp: i32,
    pub t0: i32,
    pub t1: i32,
    pub t2: i32,
    pub t3: i32,
    pub t4: i32,
    pub t5: i32,
    pub t6: i32,
    pub a0: i32,
    pub a1: i32,
    pub a2: i32,
    pub a3: i32,
    pub a4: i32,
    pub a5: i32,
    pub a6: i32,
    pub a7: i32,
    pub s0: i32,
    pub s1: i32,
    pub s2: i32,
    pub s3: i32,
    pub s4: i32,
    pub s5: i32,
    pub s6: i32,
    pub s7: i32,
    pub s8: i32,
    pub s9: i32,
    pub s10: i32,
    pub s11: i32,
    pub sp: i32,
}

pub const SCAUSE_ECALL: i32 = 8;

#[no_mangle]
pub unsafe extern "C" fn handle_trap(trap_frame: *const trap_frame) {
    let scause = read_csr!("scause");
    let stval = read_csr!("stval");
    let mut user_pc = read_csr!("sepc");

    if scause == SCAUSE_ECALL {
        handle_syscall(trap_frame);
        user_pc +=4;
    } else {
        println!(
            "unexpected trap scause={:x}, stval={:x}, sepc={:x}",
            scause, stval, user_pc
        );

        loop {}
    }

    write_csr!("sepc", user_pc);
}

pub const SYS_PUTCHAR: i32 = 1;
pub unsafe fn handle_syscall(trap_frame: *const trap_frame) {
    // let frame = *trap_frame;

    match (*trap_frame).a3  {
        SYS_PUTCHAR => {
            putchar(((*trap_frame).a0 as u8 & 0xff) as char);
            return;
        }
        _ => {
            panic!()
        }
    }

}