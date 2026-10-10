#![no_std]
#![no_main]

// I want to boot into QEMU and reach a Rust entry function
//
//
//

use bootloader_api::{entry_point, BootInfo};// here i can change the stack ize and if i make a config
use core::panic::PanicInfo;

entry_point!(boottest); // this is a macro to set my entry point for my bootloader api 

fn boottest(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut()
    {
        framebuffer.buffer_mut().fill(0x00);
        let info = framebuffer.info();
        for pixel in framebuffer
            .buffer_mut()
            .chunks_exact_mut(info.bytes_per_pixel)
            {
                pixel[0] = 0x80;
                pixel[1] = 255;
            }
    }
    loop
    {
        core::hint::spin_loop();
    }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    loop{
        core::hint::spin_loop();
    }
}
