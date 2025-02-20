#![no_std]
#![no_main]
#![cfg_attr(feature = "experimental", feature(asm_experimental_arch))]

use core::{arch::asm, panic::PanicInfo};
use lib_linux::{exit, Args};
use start_linux::start_linux;

#[panic_handler]
fn panic_handler(_pi: &PanicInfo) -> ! {
    lib_linux::exit(255)
}

extern "C" {
    static _start: usize;
}

fn jump_to_payload(init: Args, entry: usize) -> ! {
    unsafe {
        #[cfg(target_arch = "aarch64")]
        asm!(
            "mov sp, {init}",
            "br {start}",
            options(noreturn),
            init = in(reg) init.get(),
            start = in(reg) entry,
        );

        #[cfg(target_arch = "x86_64")]
        asm!(
            "mov {init}, %rsp",
            "jmp *{start}",
            options(att_syntax, noreturn),
            init = in(reg) init.get(),
            start = in(reg) entry,
        );
    }
}

#[start_linux]
fn main(init: Args) -> ! {
    if let Some(entry) = init.envp().entry() {
        if entry == unsafe { &_start as *const usize as usize } {
            lib_linux::STDERR.write_all(b"No program to load\n");
            exit(0)
        }
        lib_linux::STDERR.write_all(b"Hello from the loader\n");
        jump_to_payload(init, entry);
    } else {
        lib_linux::STDERR.write_all(b"Missing required auxv\n");
        exit(1)
    }
}
