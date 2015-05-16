[BITS 32]
global start
start:
    mov esp, _sys_stack
    jmp stublet

ALIGN 4
mboot:
    MULTIBOOT_PAGE_ALIGN	equ 1<<0
    MULTIBOOT_MEMORY_INFO	equ 1<<1
    MULTIBOOT_AOUT_KLUDGE	equ 1<<16
    MULTIBOOT_HEADER_MAGIC	equ 0x1BADB002
    MULTIBOOT_HEADER_FLAGS	equ MULTIBOOT_PAGE_ALIGN | MULTIBOOT_MEMORY_INFO | MULTIBOOT_AOUT_KLUDGE
    MULTIBOOT_CHECKSUM	equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)
    EXTERN code, bss, end

    dd MULTIBOOT_HEADER_MAGIC
    dd MULTIBOOT_HEADER_FLAGS
    dd MULTIBOOT_CHECKSUM
    
    dd mboot
    dd code
    dd bss
    dd end
    dd start

stublet:
    cli
    lgdt [gdtr]
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x08:k_main

k_main:
    extern setup
    call setup
    sti
    extern main
    call main
    jmp $
    
gdtr:
	dw gdt - gdt_end  ;might need to add 1
	dd gdt
gdt:
    dq 0
    ;CS
    dw 0xffff
    dw 0x0000
    db 0x00
    db 0b10011010
    db 0x4f
    db 0x00
    ;DS
    dw 0xffff
    dw 0x0000
    db 0x00
    db 0b10010010
    db 0x4f
    db 0x00
gdt_end:

global idt_load
extern _idtr
idt_load:
    lidt [_idtr]
    ret

%include "isr.asm"

SECTION .bss
    resb 8192
_sys_stack:

