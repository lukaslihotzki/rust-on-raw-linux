#![no_std]
#![no_main]
#![cfg_attr(feature = "experimental", feature(asm_experimental_arch))]

use core::panic::PanicInfo;
use lib_linux::{exit, Args, STDOUT};
use start_linux::start_linux;

#[panic_handler]
fn panic_handler(_pi: &PanicInfo) -> ! {
    lib_linux::exit(255);
}

#[start_linux]
fn main(init: Args) -> ! {
    let mut init = init.into_args_mut();
    for arg in init.args().skip(1) {
        let Some(last) = arg.last_mut() else { continue };
        *last = b' ';
    }
    let args = init.arg_buf(1);
    let result = if let Some(terminator) = args.last_mut() {
        *terminator = b'\n';
        STDOUT.write_all(args)
    } else {
        STDOUT.write_all(b"\n")
    };
    exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    })
}
