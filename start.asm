[BITS 32]
global start
global __morestack
;%include "gdt.asm"

BASE_ADDR equ 0x0100000

start:
    cli
    mov esp, _sys_stack - BASE_ADDR
    mov [gs:0x30], dword 0
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
    xor ax, ax
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    sti
    lgdt [gdtr]
    jmp 0x8:finish_start

finish_start:
    mov ax, 0x10
    mov ds, ax
    mov ss, ax
    mov es, ax
    extern setup
    call setup
    extern main
    call main
    jmp $
    
global idt_load
idt_load:
    mov eax, [esp+4]
    lidt [eax]
    ret


__morestack:
    jmp $

%include "isr.asm"
SECTION .data
global gdtr
gdtr:
    dw gdt_bottom - gdt -1
    dd gdt
gdt:
    ; Null
    dd 0
    dd 0

    ; Code
    dw 0xFFFF
    dw 0
    db 0
    db 10011010b
    db 11001111b
    db 0

    ; Data
    dw 0xFFFF
    dw 0
    db 0
    db 10010010b
    db 11001111b
    db 0
gdt_bottom:
SECTION .bss
    resb 16384
_sys_stack:

