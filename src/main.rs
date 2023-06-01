#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}",1);
    println!("humm??");
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

