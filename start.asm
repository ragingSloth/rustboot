[BITS 32]
%include "isr.asm"
extern main
extern _load_idt

global start
start:
    mov esp, _sys_stack
    jmp stub

global idtr
idtr:
    dw 0
    dd 0

ALIGN 4
mboot:
    MB_PAGE_ALIGN    equ 1<<0
    MB_MEMORY_INFO   equ 1<<1
    MB_AOUT_KLUDGE   equ 1<<16
    MB_HEADER_MAGIC  equ 0x1BADB002
    MB_HEADER_FLAGS  equ MB_PAGE_ALIGN | MB_MEMORY_INFO | MB_AOUT_KLUDGE
    MB_CHECKSUM  equ -(MB_HEADER_MAGIC + MB_HEADER_FLAGS)
    EXTERN code, bss, end

    dd MB_HEADER_MAGIC
    dd MB_HEADER_FLAGS
    dd MB_CHECKSUM

    dd mboot
    dd code
    dd bss
    dd end
    dd start
stub:
    ;call _load_idt
    ;lidt [idtr]
    ;sti
    jmp load_gdt
    ;jmp $

main_wrapper:
    sti
    call main
    jmp $

load_gdt:
    mov eax, 2 << 3
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax
    cli
    lgdt [gdtr]
    jmp (1 << 3):main_wrapper

gdtr:
    dw (gdt_end - gdt) + 1
    dd gdt

gdt:
    dq 0
   ;CS 
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10011010   ; access byte - code
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
    ;DS
    dw 0xffff       ; limit 0:15
    dw 0x0000       ; base 0:15
    db 0x00         ; base 16:23
    db 0b10010010   ; access byte - data
    db 0x4f         ; flags/(limit 16:19). flag is set to 32 bit protected mode
    db 0x00         ; base 24:31
gdt_end:

SECTION .bss
    resb 8192
_sys_stack:
