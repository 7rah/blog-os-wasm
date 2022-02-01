#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

use blog_os::{hit_loop, memory, println, serial_println};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::{
    structures::paging::{Page, Translate},
    VirtAddr,
};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("[Kernel] Hello world{}", "!");
    blog_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    let mut frame_allocator =
        unsafe { memory::BoootInfoFrameAllocator::init(&boot_info.memory_map) };
    //let mut frame_allocator = memory::EmptyFrameAllocator;
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    println!("{:?}", page_ptr);

    //write new!
    unsafe {
        page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e);
    }

    //use blog_os::memory::active_level_4_table;
    //let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    //let level_4_table = unsafe { active_level_4_table(phys_mem_offset) };
    //for i in 0..512 {
    //    if !level_4_table[i].is_unused() {
    //        serial_println!("Entry {}: {:?}", i, level_4_table[i]);
    //    }
    //}

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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hit_loop()
}
