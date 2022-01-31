#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{hit_loop, println, serial_println};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("[Kernel] Hello world{}", "!");
    serial_println!("[Kernel] Hello world{}", "!");
    blog_os::init();

    use x86_64::structures::paging::PageTable;

    let level_4_table_ptr = 0x1000 as *const PageTable;
    let level_4_table = unsafe { &*level_4_table_ptr };
    for i in 0..512 {
        if !level_4_table[i].is_unused() {
            serial_println!("Entry {}: {:?}", i, level_4_table[i]);
        }
    }

    //use x86_64::registers::control::Cr3;
    //let (level_4_pagetable, _) = Cr3::read();
    //println!(
    //    "level 4 page table at: {:?}",
    //    level_4_pagetable.start_address()
    //);

    //let ptr = 0x203fb1 as *mut u32;
    //unsafe { let x = *ptr;println!("{x}"); }
    //unsafe {*ptr = 42; }
    println!("It did not crash");

    #[cfg(test)]
    test_main();

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
