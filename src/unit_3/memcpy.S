.section .text
.global memcpy

memcpy:
    # a0 = 源地址
    # a1 = 目标地址

    # 加载实际地址
    mv t0, a0            # t0 = 0x80200000
    mv t1, a1            # t1 = 0x80210000

    li t2, 32               # 复制 32 字节
    add t2, t0, t2          # 结束地址

_loop_memcpy:
    ld t3, 0(t0)            # 加载数据
    sd t3, 0(t1)            # 存储数据
    addi t0, t0, 8
    addi t1, t1, 8
    blt t0, t2, _loop_memcpy

    ret