%macro ISR_RET 1
    global _isr%1
    _isr%1:
        cli
        push byte 0
        push byte %1
        jmp isr_stub
%endmacro

%macro ISR_NO_RET 1
    global _isr%1
    _isr%1:
        cli
        push byte %1
        jmp isr_stub
%endmacro

ISR_RET 0
ISR_RET 1
ISR_RET 2
ISR_RET 3
ISR_RET 4
ISR_RET 5
ISR_RET 6
ISR_RET 7
ISR_NO_RET 8
ISR_RET 9
ISR_NO_RET 10
ISR_NO_RET 11
ISR_NO_RET 12
ISR_NO_RET 13
ISR_NO_RET 14
ISR_RET 15
ISR_RET 16
ISR_RET 17
ISR_RET 18
ISR_RET 19
ISR_RET 20
ISR_RET 21
ISR_RET 22
ISR_RET 23
ISR_RET 24
ISR_RET 25
ISR_RET 26
ISR_RET 27
ISR_RET 28
ISR_RET 29
ISR_RET 30
ISR_RET 31

extern fault_handler
isr_stub:
    mov edx, 0x808
    pushad

    mov ax, ds
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov eax, esp
    push eax
    
        mov eax, fault_handler
        call eax 


    pop eax
    pop eax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    popad
    add esp, 8
    sti
    iret


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;;IRQs
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
%macro IRQ 2
    global _irq%1
    _irq%1:
        cli
        push byte 0
        push byte %2
        jmp isr_stub
%endmacro

IRQ 0,32
IRQ 1, 33
IRQ 2, 34
IRQ 3, 35
IRQ 4, 36
IRQ 5, 37
IRQ 6, 38
IRQ 7, 39
IRQ 8, 40
IRQ 9, 41
IRQ 10, 42
IRQ 11, 43
IRQ 12, 44
IRQ 13, 45
IRQ 14, 46
IRQ 15, 47
