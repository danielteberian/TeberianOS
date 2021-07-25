//Disables the use of any standard libraries. Project Autonomy is working!
#![no_std]
//Disables the use of the normal entry point chain.
#![no_main]

//Instructs the kernel to use the included panic handler.
use core::panic::PanicInfo;

mod vga_buffer;

//The first thing that my operating system has ever done.
static ALIVE: &[u8] = b"I am alive!";

//This is a replacement for the main function.
#[no_mangle] //Leave the name of this function alone! >:(
pub extern "C" fn _start() -> ! {
	use core::fmt::Write;
	vga_buffer::WRITER.lock().write_str("I am still alive.").unwrap();
	write!(vga_buffer::WRITER.lock(), " Here are some cool numbers: {} {} {}", 69, 420, 1337).unwrap();

	loop {}
}


//This is used in the event of a panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}
