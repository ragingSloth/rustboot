SHELL:=/usr/bin/env bash
LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
RUSTCFLAGS= -L . -O -g --target i686-unknown-linux-gnu --crate-type lib -C relocation-model=static -Z no-landing-pads
#RUSTCFLAGS= -L . -O --target i686-unknown-linux-gnu --crate-type lib  -Z no-landing-pads

.SUFFIXES: .o .rs .asm
.PHONY: clean run reset bin

all: kernel.bin

.asm.o:
	$(NASM) -f elf32 -o $@ $<

libcore.rlib: 
	$(RUSTC) $(RUSTCFLAGS) libcore/lib.rs

main.o: main.rs libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) -o $@ --emit obj $<

kernel.bin: start.o main.o libcore.rlib
	$(LD) -melf_i386 -Tlink.ld -o $@ $^ 


clean:
	rm -rf *.o *.iso kernel.bin

reset:
	rm -rf *.o *.iso kernel.bin *.rlib

run: test.iso
	$(QEMU) -cdrom $<
bin: kernel.bin
	$(QEMU) -kernel $<


test.iso: kernel.bin
	./build_image.sh
