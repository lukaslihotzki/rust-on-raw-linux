#!/bin/sh -e
ARCH=${ARCH:-$(rustc -vV | sed -n 's/-.*//;s/^host: //p')}

if rustc -vV | grep -q nightly; then
    cargo build -p utilities  --features experimental --target "$ARCH"-unknown-none --release -Z build-std=core,panic_abort -Z build-std-features=panic_immediate_abort
else
    cargo build -p utilities --target "$ARCH"-unknown-none --release
fi

sstrip ./target/aarch64-unknown-none/release/*
