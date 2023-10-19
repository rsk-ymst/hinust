#[link_section = ".text.start"]
#[no_mangle]
pub extern "C" fn start() {
    unsafe {
        asm!(
            "lui sp, %hi(__stack_top)",
            "addi sp, sp, %lo(__stack_top)",
            "call main",
            "call exit",
            options(nostack)
        );
    }
}

#[no_mangle]
pub extern "C" fn exit() -> ! {
    loop {}
}
