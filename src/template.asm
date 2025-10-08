BITS 64
section .text
    align 16
    global _start

; # Lambda Compiler [version 0.0.1; prealpha]
; For x86_64 GNU/Linux only (tested in Ubuntu)
; (a) Anti-Copyrights 2025, Remilia Darknets

; RAX = value and callee lambda
; RBX = argument when applying
; RCX-R15 = binding environment

; Source
; $src

_start:
$main
    mov rbx, church_decode
    call rax        ; Partial apply
    mov rbx, 0      ; Initial value
    call rax        ; Decode to integer

    mov rdi, rax    ; Return evaluated value
    mov rax, 60     ; Exit
    syscall

$code

church_decode:
    mov rax, rbx
    add rax, 1
    ret
