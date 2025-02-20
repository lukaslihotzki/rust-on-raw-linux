#!/bin/sh -ev
cd target/$(rustc -vV | sed -n 's/-.*//;s/^host: //p')-unknown-none/release
run() {
    "./$@"
}

run true
! run false
[ "$(run echo hello world)" = "hello world" ]
[ "$(echo hello world | run cat)" = "hello world" ]
[ "$(echo hello world | run cat -)" = "hello world" ]
[ "$(echo hello world | run cat - -)" = "hello world" ]
[ "$(echo hello world | run cat /dev/null)" = "" ]
BEFORE=$(date +%s%N)
run sleep 1.2
AFTER=$(date +%s%N)
[ $((AFTER-BEFORE)) -gt 1200000000 ]

# tests passed
