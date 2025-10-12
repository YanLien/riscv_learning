.section .text
.global memset

memset:
    # a0 = 目标地址（需要返回）
    # a1 = 填充值
    # a2 = 字节数
    
    mv t0, a0               # 保存原始地址
    beqz a2, _done_memset    # 如果 n == 0，直接返回
    
    add a2, a0, a2          # a2 = 结束地址（a0 + n）

_loop_memset:
    sb a1, 0(a0)            # 存储字节
    addi a0, a0, 1          # 移动到下一字节
    blt a0, a2, _loop_memset # 如果还没到结束地址，继续

_done_memset:
    mv a0, t0               # 恢复原始地址作为返回值
    ret