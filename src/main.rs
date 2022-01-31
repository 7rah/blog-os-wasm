#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{hit_loop, print, println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[Kernel] Hello world{}", "!");
    blog_os::init();
    print!("[Kernel] Hello world again{}", "!");
    //x86_64::instructions::interrupts::int3();

    //fn stack() {
    //    stack();
    //}
    //
    //stack();

    #[cfg(test)]
    test_main();
    //blog_os::vga_buffer::print_something();

    hit_loop()
}

#[allow(unused)]
fn main() {
    //println!("Hello, world!");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hit_loop()
}
