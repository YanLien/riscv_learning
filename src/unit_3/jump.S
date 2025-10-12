.section .text

.global compare_and_return
.global sel_test

compare_and_return:
    # a0 = 第一个整数
    # a1 = 第二个整数

    bge a0, a1, .L_greater_equal    # 如果 a0 >= a1，跳转到 .L_greater_equal
    li a0, -1                       # 否则，返回 -1
    ret

    .L_greater_equal:
        li a0, 0                    # 返回 0
    ret

sel_test:
    # a0 = 第一个整数
    # a1 = 第二个整数

    beqz a0, .L_sel_a0           # 如果 a0 == 0，跳转到 .L_sel_a0
    addi a1, a1, -1                 # 否则，a1 - 1
    mv a0, a1                       # 否则，返回 a1
    ret

    .L_sel_a0:
        addi a1, a1, 2             # a1 + 2
        mv a0, a1                   # 返回 a1
    ret
