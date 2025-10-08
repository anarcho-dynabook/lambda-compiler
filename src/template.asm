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
	call rax
	mov rbx, 0
	call rax

	mov rdi, rax
	mov rax, 60
	syscall

$code

church_decode:
    mov rax, rbx
	add rax, 1
	ret
