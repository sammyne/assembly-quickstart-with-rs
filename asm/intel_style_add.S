.intel_syntax noprefix

.text

.globl intel_style_add

# note the order of src and dst
intel_style_add:
    addq rsi, rdi 
    movq rax, rsi
    ret