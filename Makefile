SHELL:=/usr/bin/env bash
LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
RUSTCFLAGS= -L . -O --target i686-unknown-linux-gnu --crate-type lib -C relocation-model=static

.SUFFIXES: .o .rs .asm
.PHONY: clean run reset

all: kernel.bin

.asm.o:
	$(NASM) -f aout -o $@ $<

libcore.rlib: 
	$(RUSTC) $(RUSTCFLAGS) libcore/lib.rs

main.o: main.rs libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) -o $@ --emit obj $<

kernel.bin: start.o main.o 
	$(LD) -melf_i386 -Tlink.ld -o $@ $^


clean:
	rm -rf *.o *.iso kernel.bin

reset:
	rm -rf *.o *.iso kernel.bin *.rlib

run: test.iso
	$(QEMU) -cdrom $<
#run: kernel.bin
#	$(QEMU) -kernel $<


test.iso: kernel.bin
	./build_image.sh
