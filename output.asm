.text:
	align 16
	global _main
_main:
	lea rax, [rel LA.0]
	mov rbx, rax
	lea rax, [rel LA.1]
	call rax

	mov rdi, rax
	mov rax, 0x2000001
	syscall

LA.0:	mov rbx, rdx
	mov rax, rbx
	ret

LA.1:	mov rbx, rdx
	mov rax, rbx
	mov rbx, rax
	mov rax, rbx
	call rax
	ret


