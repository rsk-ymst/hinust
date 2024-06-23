#![no_std]

use core::panic::PanicInfo;
use mystd;

#[no_mangle]
pub fn main() {
    loop {}
}



#[panic_handler]
#[no_mangle]
pub fn dummy_panic_shell(info: &PanicInfo) -> ! {
    // 何もせず、無限ループする
    // println!("{}", info);
    loop{}
}
