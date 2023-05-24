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
    sub rsp, 72

    mov rax, 2
    mov [rsp + 0], rax
    mov rax, 4
    mov [rsp + 8], rax
    mov rax, 6
    mov [rsp + 16], rax
    mov rax, 8
    mov [rsp + 24], rax
    mov rax, 10
    mov [rsp + 32], rax
    mov rax, [rsp + 32]
    mov [r15 + 40], rax
    mov rax, [rsp + 24]
    mov [r15 + 32], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 10
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 48
    sub rsp, 8
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 8
    mov rax, 2
    mov [rsp + 0], rax
    mov rax, 4
    mov [rsp + 8], rax
    mov rax, 6
    mov [rsp + 16], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 6
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 32
    mov [rsp + 0], rax
    mov rax, 8
    mov [rsp + 8], rax
    mov rax, 10
    mov [rsp + 16], rax
    mov rax, 12
    mov [rsp + 24], rax
    mov rax, 14
    mov [rsp + 32], rax
    mov rax, 16
    mov [rsp + 40], rax
    mov rax, [rsp + 40]
    mov [r15 + 16], rax
    mov rax, [rsp + 32]
    mov [r15 + 8], rax
    mov rax, 4
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 24
    mov [rsp + 32], rax
    mov rax, 18
    mov [rsp + 40], rax
    mov rax, [rsp + 40]
    mov [r15 + 48], rax
    mov rax, [rsp + 32]
    mov [r15 + 40], rax
    mov rax, [rsp + 24]
    mov [r15 + 32], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 12
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 56
    sub rsp, 8
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 8
    mov rax, 20
    mov [rsp + 0], rax
    mov rax, 40
    mov [rsp + 8], rax
    mov rax, 60
    mov [rsp + 16], rax
    mov rax, 80
    mov [rsp + 24], rax
    mov rax, 100
    mov [rsp + 32], rax
    mov rax, [rsp + 32]
    mov [r15 + 40], rax
    mov rax, [rsp + 24]
    mov [r15 + 32], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 10
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 48
    mov [rsp + 0], rax
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 2
    mov rbx, [rsp + 8]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 8]
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
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]

    add rsp, 72
    ret
