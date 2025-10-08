BITS 64
section .text
	align 16
	global _start

; # Lambda Compiler ver.0.1
; RAX = value and callee lambda
; RDX = argument when applying
; RCX-R15 = binding environment

; Source
; (λx.λf.fx) (λf.λx.f(f(fx))) (\x.x)


_start:
	mov rax, LA.0	; Store address of lambda
	mov rbx, rax	; Argument: (λx.x)
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, LA.1	; Store address of lambda
	mov rbx, rax	; Argument: (λf.(λx.(f(f(fx)))))
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, LA.3	; Store address of lambda
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: (λx.(λf.(fx)))
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: ((λx.(λf.(fx)))(λf.(λx.(f(f(fx))))))

	mov rbx, church_decode
	call rax        ; Partial apply
	mov rbx, 0      ; Initial value
	call rax        ; Decode to integer

	mov rdi, rax    ; Return evaluated value
	mov rax, 60     ; Exit (in the Linux only)
	syscall

LA.0:
	; Lambda Abstract: (λx.x)
	; Environment { x: rcx }
	mov rcx, rbx	; Bind variable: x
	mov rax, rcx	; Load variable x
	ret

LA.2:
	; Lambda Abstract: (λx.(f(f(fx))))
	; Environment { f: rcx, x: rdx }
	mov rdx, rbx	; Bind variable: x
	mov rax, rdx	; Load variable x
	mov rbx, rax	; Argument: x
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, rcx	; Load variable f
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: f
	mov rbx, rax	; Argument: (fx)
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, rcx	; Load variable f
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: f
	mov rbx, rax	; Argument: (f(fx))
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, rcx	; Load variable f
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: f
	ret

LA.1:
	; Lambda Abstract: (λf.(λx.(f(f(fx)))))
	; Environment { f: rcx }
	mov rcx, rbx	; Bind variable: f
	mov rax, LA.2	; Store address of lambda
	ret

LA.4:
	; Lambda Abstract: (λf.(fx))
	; Environment { x: rcx, f: rdx }
	mov rdx, rbx	; Bind variable: f
	mov rax, rcx	; Load variable x
	mov rbx, rax	; Argument: x
	push rbx		; Migrate to stack (protect from rewrite)
	mov rax, rdx	; Load variable f
	pop rbx			; Reinstate in argument
	call rax		; Apply lambda: f
	ret

LA.3:
	; Lambda Abstract: (λx.(λf.(fx)))
	; Environment { x: rcx }
	mov rcx, rbx	; Bind variable: x
	mov rax, LA.4	; Store address of lambda
	ret



church_decode:
    mov rax, rbx
	add rax, 1
	ret

