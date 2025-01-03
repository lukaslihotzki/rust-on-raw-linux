#!/bin/sh -e
ARCH=${ARCH:-$(rustc -vV | sed -n 's/-.*//;s/^host: //p')}

if rustc -vV | grep -q nightly; then
    cargo build -p utilities --features experimental --target "$ARCH"-unknown-none --release -Z build-std=core,panic_abort -Z build-std-features=panic_immediate_abort
else
    cargo build -p utilities --target "$ARCH"-unknown-none --release
fi

if command -v sstrip >/dev/null; then
    set -- cat echo false sleep true
    mkdir -p target/"$ARCH"-unknown-none/sstrip
    cp ${@/#/target/"$ARCH"-unknown-none/release/} target/"$ARCH"-unknown-none/sstrip
    sstrip ${@/#/target/"$ARCH"-unknown-none/sstrip/}
fi
