#!/bin/bash
set -x
source tools/_vars.sh
rust-nm $GOOSE_ELF
