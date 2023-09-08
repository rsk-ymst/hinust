#!/bin/bash
set -xue
PATH=$PATH:/opt/riscv/bin

QEMU=qemu-system-riscv32
TARGET_LIB=./target/riscv32imac-unknown-none-elf/debug/libos_dev.a
SWITCH=./src/switch.S


DEBUG_DIR=./dev
ELF_DIR=./elf

# GCCのパス (Ubuntuの場合は CC=clang)
GCC=riscv32-unknown-linux-gnu-gcc


cargo build

# カーネルをビルド
$GCC -T kernel.ld $TARGET_LIB $SWITCH -Wl,-Map=$DEBUG_DIR/kernel.map -o $ELF_DIR/kernel.elf -nostdlib

riscv32-unknown-linux-gnu-objdump -D $ELF_DIR/kernel.elf > $DEBUG_DIR/kernel.disasm

# # QEMUを起動
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -kernel $ELF_DIR/kernel.elf
