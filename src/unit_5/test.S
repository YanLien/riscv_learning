.include "src/unit_5/macro.S"

.section .rodata
.align 3

.global symbol_table
.global symbol_table_end

symbol_table:
    # 第一个符号
    .quad func1
    .asciz "func1"
    .align 3
    
    # 第二个符号
    .quad func2
    .asciz "func2"
    .align 3
    
    # 第三个符号
    .quad func3
    .asciz "func3"
    .align 3
    
    # 第四个符号
    .quad add_numbers
    .asciz "add_numbers"
    .align 3

symbol_table_end:
    .quad 0          # 结束标记
    .asciz ""

.section .text

# ===== 公共恢复代码块 =====
common_restore:
    ld s1, 8(sp)
    ld s0, 16(sp)
    ld ra, 24(sp)
    addi sp, sp, 32
    ret

# ===== 函数1：查找数组最大值 =====
.global find_max
.type find_max, @function

# 参数：a0 = 数组地址, a1 = 数组大小
# 返回：a0 = 最大值
find_max:
    addi sp, sp, -32
    sd ra, 24(sp)
    sd s0, 16(sp)
    sd s1, 8(sp)

    blez a1, .Lmax_empty

    lw s0, 0(a0)
    li s1, 1

.Lmax_loop:
    bge s1, a1, .Lmax_done

    slli t0, s1, 2
    add t0, a0, t0
    lw t1, 0(t0)

    ble t1, s0, .Lmax_skip
    mv s0, t1

.Lmax_skip:
    addi s1, s1, 1
    j .Lmax_loop

.Lmax_done:
    mv a0, s0
    j common_restore

.Lmax_empty:
    li a0, 0
    j common_restore

.size find_max, .-find_max

# ===== 函数2：比较两个数并返回较大值 =====
.global find_two_max
.type find_two_max, @function

# 参数：a0 = 第一个数, a1 = 第二个数
# 返回：a0 = 较大的数
find_two_max:
    # 如果 a0 >= a1，返回 a0（已经在 a0 中）
    bge a0, a1, .Ltwo_done

    mv a0, a1

.Ltwo_done:
    ret

.size find_two_max, .-find_two_max

.global compare_num
.type compare_num, @function

compare_num:
    addi sp, sp, -16
    sd ra, 8(sp)

    call compare_number

    ld ra, 8(sp)
    addi sp, sp, 16
    ret

.global func1
func1:
    li a0, 1
    ret

.global func2
func2:
    li a0, 2
    ret

.global func3
func3:
    li a0, 3
    ret

.global add_numbers
add_numbers:
    add a0, a0, a1
    # add a0, a0, a2
    ret

.global macro_test
macro_test:
    addi sp, sp, -16
    sd ra, 8(sp)

    op_func 1, 10, 20    # 调用 add_1

    ld ra, 8(sp)
    addi sp, sp, 16
    ret
