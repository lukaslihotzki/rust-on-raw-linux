# rust-on-raw-linux

This workspace demonstrates how to use Rust to create useful, statically linked
binaries for Linux without libc.

To avoid using any libc, an `*-unknown-none` target is used. For making system
calls, the existing `syscalls` crate is used, which provides the necessary
platform-specific assembly code.

Before the `main` function, additional platform-specific assembly code is
required to access the command-line arguments. This code is provided by the
`start-linux` crate in this workspace. The assembly code for each architecture
is taken from musl and wrapped in Rust code by `./generate-start-linux.sh`. An
attribute originally defined in `start-linux-attr` is re-exported that easily
adds the startup code to a main function.

Consequently, this workspace is expected to run on any CPU architecture that is
supported by the syscalls crate and musl. Architectures where inline assembly
is gated by the `asm_experimental_arch` feature require a nightly toolchain.

The `lib-linux` crate is intended to provide a safe abstraction over
`start-linux` and `syscalls`, through it only implements a small subset of the
possible functionality. The safety of the crate has not been verified.

The `utilities` crate provides the binaries `true`, `false`, `sleep`, `echo`,
and `cat`, which mimic their respective POSIX utilities. This crate uses safe
code only. The binaries indicate errors only by the exit code, because they
never output any diagnostic messages.

Use the `./build.sh` script to build the utilities. Set the `ARCH` environment
variable for cross-compiling (e.g., `ARCH=aarch64 ./build.sh`). The script
automatically enables some refinements when it detects a nightly toolchain.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
