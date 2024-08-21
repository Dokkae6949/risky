use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panicked at {}", info.location().unwrap());

    loop {}
}
