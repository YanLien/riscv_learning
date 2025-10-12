# unit_3/load.S
.section .data
test_data_4bytes:
    .word 0x12345678
test_data_8bytes:
    .dword 0xabcdef0123456789

.section .text
.global load

load:
    # a0 = address of result array
    
    # Load test data from data section
    li t2, 0x80200000      # Load base address to t2
    
    la t6, test_data_4bytes
    lw t6, 0(t6)
    sw t6, 0(t2)           # Write 4 bytes to 0x80200000
    
    la t6, test_data_8bytes
    ld t6, 0(t6)
    sd t6, 16(t2)          # Write 8 bytes to 0x80200010
    
    # (1)-(3) Previous exercises
    li t3, 16              # Load immediate 16 to t3
    
    lwu t4, 0(t2)          # Read 4 bytes from 0x80200000
    ld t5, 16(t2)          # Read 8 bytes from 0x80200010
    
    # (4) LUI instruction execution
    lui t0, 0x8034f        # t0 = 0xffffffff8034f000
    lui t1, 0x400          # t1 = 0x0000000000400000
    
    # Save all values to array
    sd t2, 0(a0)           # results[0] = base address
    sd t3, 8(a0)           # results[1] = immediate 16
    sd t4, 16(a0)          # results[2] = 4-byte data
    sd t5, 24(a0)          # results[3] = 8-byte data
    sd t0, 32(a0)          # results[4] = t0 (lui 0x8034f)
    sd t1, 40(a0)          # results[5] = t1 (lui 0x400)
    
    ret