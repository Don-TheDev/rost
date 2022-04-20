#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rost::test_runner)]
#![reexport_test_harness_main = "test_main"]    

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rost::{println};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rost::memory;
    use x86_64::{structures::paging::{Page, Translate}, VirtAddr};
    println!("Hello World{}", "!");

    rost::init();

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    rost::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rost::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rost::test_panic_handler(info)
}