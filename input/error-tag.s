section .text
global our_code_starts_here
extern snek_error
extern snek_print

throw_error:
    mov rdi, rbx
    push rsp
    and rsp, -16
    call snek_error
    ret



our_code_starts_here:
    mov r15, rsi
    sub rsp, 8

    mov rax, 200
    mov [rsp + 0], rax
    mov rax, 2
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]

    add rsp, 8
    ret
