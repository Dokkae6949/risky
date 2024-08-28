#![feature(const_mut_refs)]
#![no_std]
#![no_main]

extern crate alloc;

use core::arch::asm;
use crate::task::simple_executor::SimpleExecutor;
use crate::task::Task;

#[macro_use]
mod arch;
mod panic;
mod logger;
mod allocator;
mod task;

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    println!("Kernel started");

    println!("Running async test...");
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(test_async()));
    executor.run();

    loop {
        println!("Running main loop...");
        // Fake sleep
        for _ in 0..10000000 {
            unsafe { asm!("nop"); }
        }
    }
}

#[no_mangle]
pub extern "C" fn kmain_ap() -> ! {
    println!("Kernel started (AP)");

    loop {}
}

async fn async_number() -> u32 {
    42
}

async fn async_add(a: u32, b: u32) -> u32 {
    a + b
}

async fn test_async() {
    let result = async_number().await;
    let result = async_add(result, 10).await;
    println!("Async number: {}", result);
}