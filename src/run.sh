#!/bin/bash
set -xue

# QEMU file path
QEMU=qemu-system-riscv32

# Start QEMU
TOY_OS=$PWD/../target/riscv32imac-unknown-none-elf/release/riscv-toy-OS 
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot -kernel $TOY_OS
