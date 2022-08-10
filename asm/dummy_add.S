# intel style is the default for rust's global_asm macro

.text

.globl dummy_add

# note the order of src and dst
dummy_add:
    add rsi, rdi 
    mov rax, rsi
    ret