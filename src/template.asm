BITS 64
section .text
	align 16
	global _start

; # Lambda Compiler ver.0.1
; RAX = value and callee lambda
; RDX = argument when applying

; Source
; $src

_start:
$main
	mov rbx, church_decode
	call rax        ; Partial apply
	mov rbx, 0      ; Initial value
	call rax        ; Decode to integer

	mov rdi, rax    ; Return evaluated value
	mov rax, 60     ; Exit (in the Linux only)
	syscall

$code

church_decode:
    mov rax, rbx
	add rax, 1
	ret
