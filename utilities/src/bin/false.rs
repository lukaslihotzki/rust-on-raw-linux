#![no_std]
#![no_main]
#![cfg_attr(feature = "experimental", feature(asm_experimental_arch))]

use core::panic::PanicInfo;
use lib_linux::{exit, Args};
use start_linux::start_linux;

#[panic_handler]
fn panic_handler(_pi: &PanicInfo) -> ! {
    loop {
        lib_linux::exit(255);
    }
}

#[start_linux]
fn main(_init: Args) -> ! {
    exit(1);
}
