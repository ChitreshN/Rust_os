#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}","!");
    
    #[cfg(test)]
    test_main();
    
    loop{}
}



#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    println!("{}",_info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    rust_os::test_panic_handler(_info)
}