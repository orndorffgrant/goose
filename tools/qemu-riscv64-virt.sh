#!/bin/bash
set -x
source tools/_vars_r64.sh
qemu-system-riscv64 -machine virt -serial stdio -display none -kernel target/kernel.img
