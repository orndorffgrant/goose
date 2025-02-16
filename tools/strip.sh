#!/bin/bash
set -x
source tools/_vars.sh
rust-objcopy --strip-all -O binary $GOOSE_ELF target/kernel.img
