use16

org 0x7c00

clear_registers:
    xor ax, ax
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
   
    mov ah, 9           ; Print "===="
    mov al, '='         ;
    mov bx, 7           ;
    mov cx, 4           ;
    int 10h             ;

    hang:                       ; Hang!
        jmp hang 

    cli
    lgdt [gdtr]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x7e00

error:
    mov si, .msg
.loop:
    lodsb
    or al, al
    jz .done
    mov ah, 0x0e
    int 0x10
    jmp .loop
.done:
    jmp $
    .msg db "could not read disk", 0

gdtr:
    dw (gdt_end - gdt) + 1
    db gdt
gdt:
    dq 0 ;now I can segfault
    ;code segment
    dw 0xFFFF;max addressable soace b0-16
    dw 0x0000;base unknown
    db 0x00;more base
    db 0b10011010;access byte 
        ;present
        ;2bit ring 0
        ;1
        ;executable = true
        ;grow up, eg a +1 = 0xe ->oxf
        ;readable
        ;cpu sets this
    db 0b01001111;limit/settings byte
        ;limit units 0 for bytes which matches 0xfffff
        ;size of protected, 1 for 32 bit
        ;2 0s
        ;0xf gets tacked onto end of limit
    dw 0x00 ; end of base
    ;data segment
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0b10010010
    db 0b01001111
    dw 0x00 
gdt_end:

times 510-($-$$) db 0
dw 0AA55h
