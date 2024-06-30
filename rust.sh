#!/bin/bash
set -xue
PATH=$PATH:/opt/riscv/bin

QEMU=qemu-system-riscv32

TARGET_LIB=./kernel/target/riscv32imac-unknown-none-elf/debug/libkernel.a
USER_LIB=./user/shell/target/riscv32imac-unknown-none-elf/debug/libshell.a

SWITCH=./kernel/src/switch.S

OBJCOPY=llvm-objcopy

# シェルをビルド
# $CC $CFLAGS -Wl,-Tuser.ld -Wl,-Map=shell.map -o shell.elf shell.c user.c common.c
# $OBJCOPY --set-section-flags .bss=alloc,contents -O binary shell.elf shell.bin
# $OBJCOPY -Ibinary -Oelf32-littleriscv shell.bin shell.bin.o


DEBUG_DIR=./dev
ELF_DIR=./elf
BIN_DIR=./bin

# GCCのパス (Ubuntuの場合は CC=clang)
GCC=riscv32-unknown-linux-gnu-gcc

cd kernel
cargo build
cd ..

cd user/shell 
cargo build
cd ../..

# シェル

# $GCC -T kernel.ld $TARGET_LIB $SWITCH -Wl,-Map=$DEBUG_DIR/kernel.map -o $ELF_DIR/kernel.elf -nostdlib


# シェルをビルド
$GCC -T ./user/user.ld $USER_LIB -Wl,-Map=$DEBUG_DIR/shell.map -o $ELF_DIR/shell.elf -nostdlib
$OBJCOPY --set-section-flags .bss=alloc,contents -O binary $ELF_DIR/shell.elf $BIN_DIR/shell.bin
$OBJCOPY -Ibinary -Oelf32-littleriscv $BIN_DIR/shell.bin $BIN_DIR/shell.bin.o


# $CC $CFLAGS -Wl,-Tkernel.ld -Wl,-Map=kernel.map -o kernel.elf \
#     kernel.c common.c shell.bin.o

# カーネルをビルド
$GCC -T kernel.ld $TARGET_LIB $SWITCH -Wl,-Map=$DEBUG_DIR/kernel.map -o $ELF_DIR/kernel.elf -nostdlib \
    $BIN_DIR/shell.bin.o


llvm-objdump -d $ELF_DIR/kernel.elf > $ELF_DIR/kernel.elf.out
llvm-objdump -d $ELF_DIR/shell.elf > $ELF_DIR/shell.elf.out
llvm-objdump -d '/home/ymst/_repo/hinust/target/riscv32imac-unknown-none-elf/debug/libos_dev.a' > $ELF_DIR/test2

riscv32-unknown-linux-gnu-objdump -D $ELF_DIR/kernel.elf > $DEBUG_DIR/kernel.disasm

# # QEMUを起動
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -drive id=drive0,file=lorem.txt,format=raw \
    -device virtio-blk-device,drive=drive0,bus=virtio-mmio-bus.0 \
    -kernel $ELF_DIR/kernel.elf