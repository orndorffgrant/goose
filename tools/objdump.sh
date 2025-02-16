#!/bin/bash
set -x
source tools/_vars.sh
rust-objdump $@ $GOOSE_ELF
