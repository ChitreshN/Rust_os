#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(rust_os::test_runner)]

use core::panic::PanicInfo;
use rust_os::println;
#[no_mangle]
pub extern "C" fn _start() -> !{
    test_main();
    loop{}
}

fn test_runner(tests: &[&dyn Fn()]){
    unimplemented!();
}

#[test_case]
fn test_println(){
    println!("test_println_output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    rust_os::test_panic_handler(info)
}
