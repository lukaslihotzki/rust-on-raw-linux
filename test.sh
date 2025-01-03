#!/bin/sh -ev
PATH=$(pwd)/target/$(rustc -vV | sed -n 's/-.*//;s/^host: //p')-unknown-none/release:$PATH
enable -n true false echo
true
! false
[ "$(echo hello world)" == "hello world" ]
[ "$(echo hello world | cat)" == "hello world" ]
[ "$(echo hello world | cat -)" == "hello world" ]
[ "$(echo hello world | cat - -)" == "hello world" ]
[ "$(echo hello world | cat /dev/null)" == "" ]
wait
BEFORE=$(date +%s%N)
sleep 1.2
AFTER=$(date +%s%N)
((AFTER-BEFORE >= 1200000000))

# tests passed
