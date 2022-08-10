.text
.globl att_style_add

att_style_add:
    addq %rdi, %rsi
    movq %rsi, %rax
    ret