#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Qemuex{
    Succcess    = 0x10,
    Failed      = 0x11,
}

pub fn exit_qemu(exit_code : Qemuex){
    use x86_64::instructions::port::Port;

    unsafe{
        let mut port = Port::new(0xf4);// iobase for isa-debug-exit device
        port.write(exit_code as u32);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}","!");
    
    #[cfg(test)]
    test_main();
    
    loop{}
}

pub trait Testable{
    fn run(&self) -> ();
}

impl<T> Testable for T where T:Fn(),{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]

fn test_runner(tests: &[&dyn Testable]){
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(Qemuex::Succcess);
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
    serial_println!("[failed]\n");
    serial_println!("Error:: {}\n",_info);
    exit_qemu(Qemuex::Failed);
    loop{}
}