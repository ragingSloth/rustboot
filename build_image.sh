#! /usr/bin/env bash

mkdir -p iso/boot/grub/
cp kernel.bin iso/boot/
echo " menuentry "test" {
        multiboot /boot/kernel.bin
}" >> iso/boot/grub/grub.cfg
grub-mkrescue -o test.iso iso
rm -rf iso
