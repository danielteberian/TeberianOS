//Disables the use of any standard libraries. Project Autonomy is working!
#![no_std]
//Disables the use of the normal entry point chain.
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(teberian_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


//Instructs the kernel to use the included panic handler.
use core::panic::PanicInfo;
mod vga_buffer;

//The first thing that my operating system has ever done.
static ALIVE: &[u8] = b"I am alive!";


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
	println!("RUNNING {} TESTS", tests.len());
	for test in tests {
		test();
	}
}


//This is a replacement for the main function.
#[no_mangle] //Leave the name of this function alone! >:(
pub extern "C" fn _start() -> ! {
	println!("Still going, but with better code{}", "!");

	#[cfg(test)]
	test_main();

	loop {}
}


//This is used in the event of a panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	println!("{}", info);
	loop {}
}
