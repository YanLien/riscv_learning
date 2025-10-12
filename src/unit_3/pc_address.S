# unit_3/pc_address.S
.section .text
.global pc_address

pc_address:
    auipc   t0, 1              # t0 = PC + 4096
    addi    t0, t0, -2048      # t0 = t0 - 2048
    ld      t1, -2048(t0)      # t1 = *(t0 - 2048)
    
    mv      a0, t0             # 返回值1: t0
    mv      a1, t1             # 返回值2: t1
    ret