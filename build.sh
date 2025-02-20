#!/bin/sh -e
ARCH=${ARCH:-$(rustc -vV | sed -n 's/-.*//;s/^host: //p')}

# PIE (at least on either loader or payload) is needed to prevent the loader overlapping with the payload.
# The kernel only considers one layer of interp, so the loader can reference itself as a loader.
# interp paths are relative to the working directory, so `cd` into the release or sstrip directory when executing any program.
export RUSTFLAGS='-Clink-args=--dynamic-linker=./loader -Clink-args=-pie'

if rustc -vV | grep -q nightly; then
    cargo build -p utilities --features experimental --target "$ARCH"-unknown-none --release -Z build-std=core,panic_abort -Z build-std-features=panic_immediate_abort
else
    cargo build -p utilities --target "$ARCH"-unknown-none --release
fi

if command -v sstrip >/dev/null; then
    set -- cat echo false sleep true loader
    mkdir -p target/"$ARCH"-unknown-none/sstrip
    cp ${@/#/target/"$ARCH"-unknown-none/release/} target/"$ARCH"-unknown-none/sstrip
    sstrip ${@/#/target/"$ARCH"-unknown-none/sstrip/}
fi
