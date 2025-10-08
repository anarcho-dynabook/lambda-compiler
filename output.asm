.text:
	align 16
	global _main
_main:
	lea rax, [rel LA.0]

	mov rdi, rax
	mov rax, 0x2000001
	syscall

LA.1:
	mov rcx, rdx
	mov rax, rcx
	mov rdx, rax
	mov rax, rcx
	call rax
	ret

LA.0:
	mov rbx, rdx
	lea rax, [rel LA.1]
	ret


