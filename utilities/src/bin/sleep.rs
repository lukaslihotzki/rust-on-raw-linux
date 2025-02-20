#![no_std]
#![no_main]
#![cfg_attr(feature = "experimental", feature(asm_experimental_arch))]

use core::panic::PanicInfo;
use lib_linux::{exit, Args};
use start_linux::start_linux;

#[panic_handler]
fn panic_handler(_pi: &PanicInfo) -> ! {
    lib_linux::exit(255);
}

#[start_linux]
fn main(init: Args) -> ! {
    let &[_, duration] = init.argv() else { exit(1) };
    let mut iter = duration.bytes();
    let Some(seconds) = (&mut iter)
        .take_while(|n| n.get() != b'.')
        .try_fold(0, |i, n| {
            let c = char::from(n.get() as u8);
            c.to_digit(10).map(|d| 10 * i + u64::from(d))
        })
    else {
        exit(1)
    };
    let mut frac_digits: u8 = 9;
    let Some(mut nanos) = iter.take(frac_digits.into()).try_fold(0, |i, n| {
        let c = char::from(n.get() as u8);
        frac_digits -= 1;
        c.to_digit(10).map(|d| 10 * i + d)
    }) else {
        exit(1)
    };
    while nanos != 0 && frac_digits != 0 {
        nanos *= 10;
        frac_digits -= 1;
    }

    let result = lib_linux::sleep(seconds, nanos);
    exit(match result {
        Ok(_) => 0,
        Err(_) => 1,
    });
}
