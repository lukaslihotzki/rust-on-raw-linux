#!/bin/sh -e

src=musl-1.2.5.tar.gz
[ -e "$src" ] || curl -LO https://git.musl-libc.org/cgit/musl/snapshot/"$src"
t="$(mktemp -d)"
trap 'rm -r "$t/"' EXIT
tar xzf "$src" -C "$t" --strip-components 1 musl-1.2.5/arch musl-1.2.5/COPYRIGHT
cat <<EOF
/*
Automatically generated file. Do not edit.
This file contains assembly from $src:
$(sed -n '/----/,/----/p' "$t"/COPYRIGHT)
*/

#![no_std]

pub use start_linux_attr::start_linux;

#[macro_export]
macro_rules! wrap_start {
    (\$start:ident) => {
EOF

for file in "$t"/arch/*/crt_arch.h; do
    arch=${file#"$t"/arch/}
    arch=${arch%%/*}
    case "$arch" in
        i386) arch=x86 ;;
    esac
    case "$arch" in
        microblaze|mipsn32|or1k|sh|x32) continue ;;
        x86*) opts=att_syntax ;;
        *) opts= ;;
    esac
    code=$(sed -n '
        # read complete file to pattern space
        :a;$bb;N;ba;:b;
        # extract code of first asm block, exit otherwise
        s/^.*__asm__(\(.*\));.*$/\1/;!b;
        # remove most of the quotes, now only constants are quoted
        s/\s*"//;s/"\s*$//;s/"\s*"//g;s/\s*\\n/\n/g;s/\\t/\t/g;s/\\\([\"]\)/\1/g;
        # replace START constant and the resulting symbol, then print
        s/"\s*START\s*"/_start/g;s/_start_c/{start}/g;p
    ' <$file)
    cat <<EOF

#[cfg(target_arch = "$arch")]
core::arch::global_asm!(r#"
$code
"#, options(${opts}), start = sym \$start);
EOF
done
printf '    }\n}\n'
