#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;
use blog_os::{hit_loop, println, wasm};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("[Kernel] Hello world!");
    blog_os::init(boot_info);

    let x = Box::new(42);
    println!("u32 value: {x} from heap!!");
    drop(x);

    wasm::run();
    println!("It did not crash");

    #[cfg(test)]
    test_main();

    hit_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hit_loop()
}
