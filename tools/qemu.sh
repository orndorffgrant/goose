#!/bin/bash
set -x
source tools/_vars.sh
qemu-system-aarch64 -machine raspi3b -serial stdio -display none -kernel target/kernel.img
