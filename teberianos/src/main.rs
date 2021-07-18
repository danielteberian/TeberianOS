//Disables the use of any standard libraries. Project Autonomy is working!
#![no_std]
//Disables the use of the normal entry point chain.
#![no_main]

//Instructs the kernel to use the included panic handler.
use core::panic::PanicInfo;


static ALIVE: &[u8] = b"I am alive!";

//This is a replacement for the main function.
#[no_mangle] //Leave the name of this function alone! >:(
pub extern "C" fn _start() -> ! {
	let vga_buffer = 0xb8000 as *mut u8;

	for (i, &byte) in ALIVE.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}

	loop {}
}


//This is used in the event of a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}


