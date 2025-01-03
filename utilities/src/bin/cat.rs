#![no_std]
#![no_main]
#![cfg_attr(feature = "experimental", feature(asm_experimental_arch))]

use core::{mem::MaybeUninit, panic::PanicInfo};
use lib_linux::{exit, Args, OwnedFd, STDIN, STDOUT};
use start_linux::start_linux;
use syscalls::Errno;

#[panic_handler]
fn panic_handler(_pi: &PanicInfo) -> ! {
    loop {
        lib_linux::exit(255);
    }
}

#[inline(always)]
fn copy(fd: &OwnedFd, code: &mut u8) -> Result<(), Errno> {
    let mut buf = [MaybeUninit::<u8>::uninit(); 4096];
    loop {
        let Ok(data) = fd.read_uninit(&mut buf) else {
            *code = 1;
            break;
        };
        if data.len() == 0 {
            break;
        }
        STDOUT.write_all(data)?;
    }
    Ok(())
}

#[start_linux]
fn main(init: Args) -> ! {
    let args = match init.argv() {
        [_, tail @ ..] => tail,
        [] => &[],
    };
    let mut code = 0;
    match args {
        [] => {
            if copy(&STDIN, &mut code).is_err() {
                exit(1);
            }
        }
        args => {
            for path in args {
                let fd;
                let mut it = path.bytes();
                let fdref = if (it.next(), it.next())
                    == (Some(core::num::NonZero::<u8>::new(b'-').unwrap()), None)
                {
                    &STDIN
                } else {
                    fd = OwnedFd::openat(None, &path, 0, 0).ok();
                    let Some(fd) = &fd else {
                        code = 1;
                        continue;
                    };
                    fd
                };
                if copy(&fdref, &mut code).is_err() {
                    exit(1);
                }
            }
        }
    }
    exit(code);
}
