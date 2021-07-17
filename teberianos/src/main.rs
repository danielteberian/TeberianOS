//Disables the use of any standard libraries. Project Autonomy is working!
#![no_std]
//Disables the use of the normal entry point chain.
#![no_main]

//Instructs the kernel to use the included panic handler.
use core::panic::PanicInfo;


//This is a replacement for the main function.
#[no_mangle] //Leave the name of this function alone! >:(
pub extern "C" fn _start() -> ! {
	loop {}
}


//This is used in the event of a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}

