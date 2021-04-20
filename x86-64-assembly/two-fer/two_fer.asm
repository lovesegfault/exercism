section .rodata
    msg_start: db "One for ", 0
    msg_end: db ", one for me.", 0
    you: db "you", 0

section .text
global two_fer
two_fer:
    mov r8, rdi
    mov rdi, rsi

    lea rsi, [rel msg_start]
    call copy
    mov rdi, rax
    ret

copy:
    movsb
    cmb byte[rdi-1], 0
    je .stop
    jmp copy
.stop:
    lea rax, [rdi - 1]
    ret
