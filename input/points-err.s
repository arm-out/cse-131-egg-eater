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

create_pt:
    sub rsp, 16

    mov rax, [rsp + 24]
    mov [rsp + 0], rax
    mov rax, [rsp + 32]
    mov [rsp + 8], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 4
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 24

    add rsp, 16
    ret

add_pt:
    sub rsp, 32

    mov rax, [rsp + 40]
    mov [rsp + 0], rax
    mov rax, 0
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 48]
    mov [rsp + 8], rax
    mov rax, 0
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov rbx, rax
    or rbx, [rsp + 0]
    and rbx, 1
    cmp rbx, 1
    mov rbx, 7
    je throw_error
    add rax, [rsp + 0]
    mov rbx, 8
    jo throw_error
    mov [rsp + 0], rax
    mov rax, [rsp + 40]
    mov [rsp + 8], rax
    mov rax, 2
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 8], rax
    mov rax, [rsp + 48]
    mov [rsp + 16], rax
    mov rax, 2
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 16]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov rbx, rax
    or rbx, [rsp + 8]
    and rbx, 1
    cmp rbx, 1
    mov rbx, 7
    je throw_error
    add rax, [rsp + 8]
    mov rbx, 8
    jo throw_error
    mov [rsp + 8], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 4
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 24

    add rsp, 32
    ret



our_code_starts_here:
    mov r15, rsi
    sub rsp, 56

    mov rax, 7
    mov [rsp + 0], rax
    mov rax, 10
    mov [rsp + 8], rax
    sub rsp, 24
    mov rbx, [rsp + 24]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 32]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call create_pt
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    mov rax, 14
    mov [rsp + 8], rax
    mov rax, 18
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call create_pt
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 8], rax
    mov rax, [rsp + 0]
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 8]
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 16], rax
    mov rax, [rsp + 8]
    mov [rsp + 24], rax
    sub rsp, 24
    mov rbx, [rsp + 40]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 48]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call add_pt
    mov rdi, [rsp + 16]
    add rsp, 24

    add rsp, 56
    ret
