.section .text

# 可选的，声明外部函数，但是推荐
.extern sel_test
.global bl_test

bl_test:
    # a0 = 第一个整数
    # a1 = 第二个整数

    addi sp, sp, -16    # 分配栈空间
    sd ra, 8(sp)        # 保存返回地址到栈

    call sel_test       # 调用 sel_test 函数

    ld ra, 8(sp)        # 从栈恢复返回地址
    addi sp, sp, 16     # 释放栈空间
    ret