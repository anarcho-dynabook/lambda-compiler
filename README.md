```nasm
BITS 64
section .text
    align 16
    global _start

; # Lambda Compiler [version 0.0.1; prealpha]
; For x86_64 GNU/Linux only (tested in Lubuntu)
; (a) Anti-Copyrights 2025, Remilia Darknets

; RAX = value and callee lambda
; RBX = argument when applying
; RCX-R15 = binding environment

; Source (the expression of Lambda calculus)
; This assembly code is generated from that
; `(λx.λf.fx) (λf.λx.f(f(fx))) (\x.x)`

_start:
    mov rax, LA.0       ; Function pointer
    mov rbx, rax        ; Argument `(λx.x)`
    push rbx            ; Retract to stack
    mov rax, LA.1       ; Function pointer
    mov rbx, rax        ; Argument `(λf.(λx.(f(f(fx)))))`
    push rbx            ; Retract to stack
    mov rax, LA.3       ; Function pointer
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `(λx.(λf.(fx)))`
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `((λx.(λf.(fx)))(λf.(λx.(f(f(fx))))))`

    mov rbx, church_decode
    call rax            ; Partial apply
    mov rbx, 0          ; Initial value
    call rax            ; Decode to integer

    mov rdi, rax        ; Return evaluated value
    mov rax, 60         ; Exit
    syscall

LA.0:
    ;;; Lambda Abstract `(λx.x)`
    ;;; Environment { x: RCX }
    mov rcx, rbx        ; Bind variable `x`
    mov rax, rcx        ; Load variable `x`
    ret

LA.2:
    ;;; Lambda Abstract `(λx.(f(f(fx))))`
    ;;; Environment { f: RCX, x: RDX }
    mov rdx, rbx        ; Bind variable `x`
    mov rax, rdx        ; Load variable `x`
    mov rbx, rax        ; Argument `x`
    push rbx            ; Retract to stack
    mov rax, rcx        ; Load variable `f`
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `f`
    mov rbx, rax        ; Argument `(fx)`
    push rbx            ; Retract to stack
    mov rax, rcx        ; Load variable `f`
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `f`
    mov rbx, rax        ; Argument `(f(fx))`
    push rbx            ; Retract to stack
    mov rax, rcx        ; Load variable `f`
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `f`
    ret

LA.1:
    ;;; Lambda Abstract `(λf.(λx.(f(f(fx)))))`
    ;;; Environment { f: RCX }
    mov rcx, rbx        ; Bind variable `f`
    mov rax, LA.2       ; Function pointer
    ret

LA.4:
    ;;; Lambda Abstract `(λf.(fx))`
    ;;; Environment { x: RCX, f: RDX }
    mov rdx, rbx        ; Bind variable `f`
    mov rax, rcx        ; Load variable `x`
    mov rbx, rax        ; Argument `x`
    push rbx            ; Retract to stack
    mov rax, rdx        ; Load variable `f`
    pop rbx             ; (overwrite-guard)
    call rax            ; Apply `f`
    ret

LA.3:
    ;;; Lambda Abstract `(λx.(λf.(fx)))`
    ;;; Environment { x: RCX }
    mov rcx, rbx        ; Bind variable `x`
    mov rax, LA.4       ; Function pointer
    ret



church_decode:
    mov rax, rbx
    add rax, 1
    ret
```
