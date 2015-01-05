section .data
gdt:
    dq 0
    ;null segment for segfaults
    dw 0xFFFF       ;lim 0:15
    dw 0            ;base 0:15
    db 0            ;base 16:23
    db 0b10011010
             ;^ maybe a 1? lets non ring0 execute valid code in ring0. might be useful
    db 0x4F
    db 0            ;base 24:31
    ;code segment :only accesable by ring 0
    dw 0xFFFF
    dw 0
    db 0
    db 0b10010010
    db 0x4F
    db 0
gdtr:
    dw ($ - gdt) + 1 ;size
    dd gdt           ;offset
section .text
load_gdt:
    cli
    mov ax, 0x10 ;number of bytes into gdtr to look for address
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    lgdt [gdtr]
    ret
