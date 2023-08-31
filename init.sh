#!/bin/bash

rustup target add riscv32imac-unknown-none-elf

curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-f
curl -LO https://github.com/qemu/qemu/raw/v8.0.4/pc-bios/opensbi-riscv32-generic-fw_dynamic.bin

PATH=$PATH:/opt/riscv/bin
