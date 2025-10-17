# RISCV基础指令集

1. RISC-V指令集有什么特点？
2. RISC-V指令编码格式可以分成几类？
3. 什么是零扩展和符号扩展？
4. 什么是PC相对寻址？
5. 假设当前PC值为`0x80200000`，分别执行如下指令，a5和a6寄存器的值分别是多少？

    ``` asm
    auipc   a5,0x2
    lui     a6, 0x2
    ```
6. 在下面的指令中，a1和t1寄存器的值分别是多少？
    ``` asm
    li t0, 0x8000008a00000000
    srai a1, t0, 1
    srli t1, t0, 1
    ```
7. 假设执行如下各条指令时当前的PC值为`0x80200000`，则下面哪些指令是非法指令？
    ``` asm 
    jal a0, 0x800fffff
    jal a0, 0x80300000
    ```

8. 请解析下面这条指令的含义。
    ``` asm
    csrrw tp, sscratch, tp
    ```

9. 在RISC-V指令集中，如何实现大范围和小范围内跳转？

## RISCV指令集介绍

RISC-V每条指令的宽度为32位（不考虑压缩扩展指令）​，包括RV32指令集以及RV64指令集。指令编码格式大致可分成如下6类：
+ R类型：寄存器与寄存器算术指令。
+ I类型：寄存器与立即数算术指令或者加载指令。
+ S类型：存储指令。
+ B类型：条件跳转指令。
+ U类型：长立即数操作指令。
+ J类型：无条件跳转指令。

![alt text](/doc/img/chapter3/img_1.png)

指令编码可以分成如下几个部分：
+ opcode（操作码）字段：位于指令编码Bit[6:0]​，用于指令的分类。
+ funct3和funct7（功能码）字段：常常与opcode字段结合在一起使用，用来定义指令的操作功能。
+ rd字段：表示目标寄存器的编号，位于指令编码的Bit[11:7]​。
+ rs1字段：表示第一源操作寄存器的编号，位于指令编码的Bit[19:15]​。
+ rs2字段：表示第二源操作寄存器的编号，位于指令编码的Bit[24:20]​。
+ imm：表示立即数。RISC-V中使用的立即数大部分是符号扩展(sign-extended)的立即数。

RV64指令集是基于寄存器加载和存储的体系结构设计，所以所有的数据加载、存储以及处理都是在通用寄存器中完成的。RISC-V一共有32个通用寄存器（通用寄存器都是64位宽的，可以处理64位宽的数据）​，即x0～x31，其中，x0寄存器的编号为0，以此类推。因此，指令编码使用5位宽(25 = 32)，即索引32个通用寄存器。

示例：`add rd, rs1, rs2`

RV64指令集中常用的符号说明如下:
+ rd：表示目标寄存器的编号，可以从x0～x31通用寄存器中选择。
+ rs1：表示源寄存器1的编号，可以从x0～x31通用寄存器中选择。
+ rs2：表示源寄存器2的编号，可以从x0～x31通用寄存器中选择。
+ ()：通常用来表示寻址模式，例如，(a0)表示以`a0`寄存器的值为基地址进行寻址。这个前面还可以加offset，表示偏移量，可以是正数或负数。**例如，8(a0)表示以a0寄存器的值为基地址，偏移8字节进行寻址。**
+ { }：表示可选项。
+ imm：表示有符号立即数。

## 加载和存储指令

### 加载指令

```
l{d|w|h|b}{u} rd,  offset(rs1)
```
相关选项的含义如下：
+ {d|w|h|b}：表示加载的数据宽度。
+ {u}：可选项，表示加载的数据为无符号数，即采用零扩展方式。如果没有这个选项，则加载的数据为有符号数，即采用符号扩展方式。
+ rd：表示目标寄存器。
+ rs1：表示源寄存器1。
+ (rs1)：表示以rs1寄存器的值为基地址进行寻址，简称rs1地址。
+ offset：表示以源寄存器的值为基地址的偏移量。offset是12位有符号数，取值范围为[−2048, 2047]​。

| 加载指令 | 数据位宽/位 | 说明 |
| -------- | -------- | ----- |
| `lb rd, offset(rs1)` | 8 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载1字节数据，经过符号扩展之后写入目标寄存器rd中 |
| `lbu rd, offset(rs1)` | 8 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载1字节数据，经过零扩展之后写入目标寄存器rd中 |
| `lh rd, offset(rs1)` | 16 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载2字节数据，经过符号扩展之后写入目标寄存器rd中 |
| `lhu rd, offset(rs1)` | 16 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载2字节数据，经过零扩展之后写入目标寄存器rd中 |
| `lw rd, offset(rs1)` | 32 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载4字节数据，经过符号扩展之后写入目标寄存器rd中 |
| `lwu rd, offset(rs1)` | 32 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载4字节数据，经过零扩展之后写入目标寄存器rd中 |
| `ld rd, offset(rs1)` | 64 | 以rs1寄存器的值为基地址，在偏移offset的地址处加载8字节数据，写入寄存器rd中 |
| `lui rd, imm` | 64 | 先把imm（立即数）左移12位，然后进行符号扩展，最后把结果写入rd寄存器中 |
  
### 存储指令

```
s{d|w|h|b} rs2,  offset(rs1)
```

相关选项的含义如下
+ `{d|w|h|b}`：表示存储的数据宽度。根据数据的位宽。
+ `rs1`：表示源寄存器1，用来表示基地址。
+ `(rs1)`：表示以`rs1`寄存器的值为基地址进行寻址，简称`rs1`地址。
+ `rs2`：表示源寄存器2，用来表示源操作数。
+ `offset`：表示以源寄存器的值为基地址的偏移量。`offset`是12位有符号数，取值范围为[−2048, 2047]​。

| 存储指令 | 数据位宽/位 | 说明 |
| ------- | ----------- | ---- |
| `sb rs2, offset(rs1)` | 8 | 把rs2寄存器的低8位宽的值存储到以rs1寄存器的值为基地址加上offset的地址处 |
| `sh rs2, offset(rs1)` | 16 | 把rs2寄存器的低16位宽的值存储到以rs1寄存器的值为基地址加上offset的地址处 |
| `sw rs2, offset(rs1)` | 32 | 把rs2寄存器的低32位宽的值存储到以rs1寄存器的值为基地址加上offset的地址处 |
| `sd rs2, offset(rs1)` | 64 | 把rs2寄存器的值存储到以rs1寄存器的值为基地址加上offset的地址处 |

## PC相对寻址

```
auipc rd, imm
```

把imm（立即数）左移12位并带符号扩展到64位后，得到一个新的立即数。这个新的立即数是一个有符号的立即数，再加上当前PC值，然后存储到rd寄存器中。由于新的立即数表示的是地址的高20位部分，并且是一个有符号的立即数，因此这条指令的寻址范围为基于当前的PC偏移量±2 GB，如图3.6所示。另外，由于这个新的立即数的低12位都是0，因此它只能寻址到与4 KB对齐的地址。涉及4 KB以内的寻址，则需要结合其他指令（如`ADDI`指令）来完成。

知道了当前的PC值和目标地址，如何计算AUIPC和ADDI指令的参数呢？

```
hi20 = (offset >> 12) + offset[11]
lo12 = offset&0xfff
```

案例1：`0x1800`

简单拆分错误
```
hi20 = 0x1800 >> 12 = 0x1
lo12 = 0x1800 & 0xfff = 0x800

验证：
auipc a0, 0x1         # a0 = PC + 0x1000
addi  a0, a0, 0x800   # a0 = a0 + (-2048)  ← 注意！0x800作为有符号数是-2048
                      # 结果 = PC + 0x1000 - 0x800 = PC + 0x800
                      # ❌ 错误！实际需要 PC + 0x1800
```

补偿逻辑:
当 lo12 被解释为负数时（bit[11] = 1），会从结果中减去一个值。为了补偿这个减法，需要在 `hi20` 上提前加 1。

```
如果 offset[11] = 1:
  lo12 将是负数（-2048 ~ -1）
  ADDI 会减去一个值
  所以 AUIPC 需要多加 0x1000（即 hi20 + 1）来补偿
```

案例2 ：`0x5678`

```
offset = 0x5678
offset[11] = 0（第11位是0，0x678 < 0x800）

hi20 = (0x5678 >> 12) + 0
     = 0x5 + 0
     = 0x5

lo12 = 0x5678 & 0xfff = 0x678

验证：
auipc a0, 0x5         # a0 = PC + 0x5000
addi  a0, a0, 0x678   # a0 = a0 + 0x678（正数）
                      # 结果 = PC + 0x5678 ✓
```

还有一条指令（LUI指令）与AUIPC指令类似。不同点在于LUI指令不使用PC相对寻址，它仅仅把立即数左移12位，得到一个新的32立即数，再带符号扩展到64位，将其存储到rd寄存器中。

`lui/auipc`是U-type

| 伪指令 | 指令组合 | 说明 |
| ------ | ------- | ---- |
| `la rd, symbol`（非PIC） | `auipc rd, delta[31:12] + delta[11]`<br>`addi rd, rd, delta[11:0]` | 加载符号的绝对地址<br>其中delta = symbol-pc |
| `la rd, symbol`（PIC） | `auipc rd, delta[31:12] + delta[11]` <br> `l[w\|d] rd, rd, delta[11:0]（rd）` | 加载符号的绝对地址<br>其中delta = GOT[symbol]-pc |
| `lla rd, symbol` | `auipc rd, delta[31:12] + delta[11]`<br>`addi rd, rd, delta[11:0]` | 加载符号的本地地址<br>其中delta = symbol-pc |
| `l[b\|h\|w\|d] rd, symbol` | auipc rd, delta[31:12] + delta[11]<br> l[b\|\h\|w\|d] rd, delta[11:0]（rd） | 加载符号的内容 |
| `s[b\|h\|w\|d] rd, symbol, rt` | auipc rt, delta[31:12] + delta[11]<br>s[b\|h\|w\|d] rd, delta[11:0]（rt） | 存储内容到符号中<br>其中rt为临时寄存器 |
| `li rd, imm` | 根据情况扩展为多条指令 | 加载立即数到rd寄存器中 | 

注：GCC有一个`- fpic`编译选项，它在生成的代码中使用相对地址，而不是绝对地址。无论共享库被加载器加载到内存的什么位置，代码都能正确执行，而不需要重定位(relocate)。若没有使用`-fpic`选项编译共享库，则当有多个程序加载此共享库时，加载器需要为每个程序重定位共享库，即根据加载到的位置重定位，这中间可能会触发写时复制机制。
在非PIC模式下，LLA和LA伪指令的行为相同，都是直接获取符号的绝对地址；而在PIC模式下，LA指令是从GOT中获取符号的地址，而LLA伪指令则是直接获取符号的绝对地址。

## 移位操作

+ `sll`：逻辑左移(shift left logical)，最高位丢弃，最低位补0。
+ `srl`：逻辑右移(shift right logical)，最高位补0，最低位丢弃。
+ `sra`：算术右移(shift right arithmetic)，最低位丢弃，最高位按照符号进行扩展。

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `sll` | `sll rd, rs1, rs2` | 逻辑左移指令 <br> 把rs1寄存器左移rs2位，将结果写入rd寄存器中 |
| `slli` | `slli rd, rs1, shamt` | 立即数逻辑左移指令 <br> 把rs1寄存器左移shamt位，将结果写入rd寄存器中 |
| `slliw` | `slliw rd, rs1, shamt` | 立即数逻辑左移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后左移shamt位，根据结果进行符号扩展后写入rd寄存器中 |
| `sllw` | `sllw rd, rs1, rs2` | 逻辑左移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后左移rs2位（取rs2寄存器低5位的值），根据结果进行符号扩展后写入rd寄存器中 |
| `sra` | `sra rd, rs1, rs2` | 算术右移指令 <br> 把rs1寄存器右移rs2位，根据rs1寄存器的旧值进行符号扩展后写入rd寄存器中 |
| `srai` | `srai rd, rs1, shamt` | 立即数算术右移指令 <br> 把rs1寄存器右移shamt位，进行符号扩展后写入rd寄存器中 |
| `sraiw` | `sraiw rd, rs1, shamt` | 立即数算术右移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后左移shamt位，根据新的源操作数进行符号扩展后写入rd寄存器中 |
| `sraw` | `sraw rd, rs1, rs2` | 算术右移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后右移rs2位（取rs2寄存器低5位的值），根据新的源操作数进行符号扩展后写入rd寄存器中 |
| `srl` | `srl rd, rs1, rs2` | 逻辑右移指令 <br> 把rs1寄存器右移rs2位，进行零扩展后写入rd寄存器中 |
| `srli` | `srli rd, rs1, shamt` | 立即数逻辑右移指令 <br> 把rs1寄存器右移shamt位，进行零扩展后写入rd寄存器中 |
| `srliw` | `srliw rd, rs1, shamt` | 立即数逻辑右移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后右移shamt位，进行符号扩展后写入rd寄存器中 |
| `srlw` | `srlw rd, rs1, rs2` | 逻辑右移指令 <br> 截取rs1寄存器的低32位作为新的源操作数，然后右移rs2位（取rs2寄存器低5位的值），进行符号扩展后写入rd寄存器中 |

## 位操作指令

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `and` | `and rd, rs1, rs2` | 与操作指令 <br> 对rs1和rs2寄存器按位进行与操作，把结果写入rd寄存器中 |
| `andi` | `andi rd, rs1, imm` | 与操作指令 <br> 对rs1寄存器和imm按位进行与操作，把结果写入rd寄存器中 |
| `or` | `or rd, rs1, rs2` | 或操作指令 <br> 对rs1寄存器和rs2寄存器按位进行或操作，把结果写入rd寄存器中 |
| `ori` | `ori rd, rs1, imm` | 或操作指令 <br> 对rs1寄存器和imm按位进行或操作，把结果写入rd寄存器中 |
| `xor` | `xor rd, rs1, rs2` | 异或操作指令 <br> 对rs1寄存器和rs2寄存器按位进行异或操作，把结果写入rd寄存器中 |
| `xori` | `xori rd, rs1, imm` | 异或操作指令 <br> 对rs1寄存器和imm按位进行异或操作，把结果写入rd寄存器中 |
| `not` | `not rd, rs` | 按位取反指令 <br> 对rs寄存器按位进行取反操作，把结果写入rd寄存器中，该指令是伪指令，内部使用 "xori rd, rs, -1" |

## 算数指令

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `add` | `add rd, rs1, rs2` | 加法指令 <br> 将rs1寄存器的值与rs2寄存器的值相加，把结果写入rd寄存器中 |
| `addi` | `addi rd, rs1, imm` | 加法指令 <br> 将rs1寄存器与imm相加，把结果写入rd寄存器中 |
| `addw` | `addw rd, rs1, rs2` | 加法指令 <br> 截取rs1和rs2寄存器的低32位数据作为源操作数并相加，结果载取低32位，最后进行符号扩展并写入rd寄存器中 |
| `addiw` | `addiw rd, rs1, imm` | 加法指令 <br> 截取rs1寄存器的低32位数据为源操作数，加上imm，对结果进行符号扩展并写入rd寄存器中 |
| `sub` | `sub rd, rs1, rs2` | 减法指令 <br> 将rs1寄存器的值减去rs2寄存器的值，把结果写入rd寄存器中 |
| `subw` | `subw rd, rs1, rs2` | 减法指令 <br> 截取rs1和rs2寄存器的低32位数据作为源操作数，然后新的rs1值减去新的rs2值，结果载取低32位，最后进行符号扩展并写入rd寄存器中 |

![alt text](/doc/img/chapter3/img_3.png)

![alt text](/doc/img/chapter3/img_4.png)

注：在GNU AS中，0x800被看作一个数值为2048的64位无符号数。如果想表示“−2048”立即数，需要使用`0xFFFF_FFFF_FFFF_F800`，因为汇编器中的立即数是按照处理器的位宽解析的。

## 比较指令

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `slt` | `slt rd, rs1, rs2` | 有符号数比较指令 <br> 比较rs1寄存器和rs2寄存器的值，如果rs1寄存器的值小于rs2寄存器的值，向rd寄存器写1，否则写0 |
| `sltu` | `sltu rd, rs1, rs2` | 无符号数比较指令 <br> 等同于slt指令，区别在于rs1寄存器的值和rs2寄存器的值为无符号数 |
| `slti` | `slti rd, rs1, imm` | 有符号数与立即数比较指令 <br> 比较rs1寄存器的值与imm，如果rs1寄存器的值小于imm，向rd寄存器写1，否则写0 |
| `sltiu` | `sltiu rd, rs1, imm` | 无符号数与立即数比较指令 <br> 如果rs1寄存器的值小于imm，向rd寄存器写1，否则写0 |

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `sltz` | `sltz rd, rs1` | 小于0则置位指令 <br> 如果rs1寄存器的值小于0，向rd寄存器写1；否则，写0 |
| `snez` | `snez rd, rs1` | 不等于0则置位指令 <br> 如果rs1寄存器的值不等于0，向rd寄存器写1；否则，写0 |
| `seqz` | `seqz rd, rs1` | 等于0则置位指令 <br> 如果rs1寄存器的值等于0，向rd寄存器写1；否则，写0 |
| `sgtz` | `sgtz rd, rs1` | 大于0则置位指令 <br> 如果rs1寄存器的值大于0，向rd寄存器写1；否则，写0 |

## 无条件跳转指令

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `jal` | `jal rd, offset` | 跳转与链接指令 <br> 跳转到数值PC + offset的地址，然后把返回地址（PC + 4）保存到rd寄存器中。offset是21位有符号数，跳转范围大约是当前PC值偏移±1MB，即PC-0x10_0000 ~ PC+0xF_FFFE |
| `jalr` | `jalr rd, offset(rs1)` | 使用寄存器的跳转指令 <br> 跳转到rs1寄存器的值为基地址且偏移offset的地址，然后把返回地址（PC + 4）保存到rd寄存器中。offset是12位有符号数，偏移范围为-2048 ~ 2047 |

JAL（Jump And Link，跳转与链接）指令使用J类型的指令编码
JALR（Jump And Link Register，跳转与链接寄存器）指令使用I类型指令编码

![JAL指令](/doc/img/chapter3/img_5.png)

![JALR指令](/doc/img/chapter3/img_6.png)

| 伪指令 | 指令组合 | 说明 |
|--------|----------|------|
| `j label` | `jal x0, offset` | 跳转到label处，不带返回地址 <br> 使用jal指令但不保存返回地址（目标寄存器为x0，即零寄存器） |
| `jal label` | `jal ra, offset` | 跳转到label处，返回地址存储在ra寄存器中 |
| `jr rs` | `jalr x0, 0(rs)` | 跳转到rs寄存器中的地址址，不带返回地址 |
| `jalr rs` | `jalr ra, 0(rs)` | 跳转到rs寄存器中的地址，返回地址存储在ra寄存器中 |
| `ret` | `jalr x0, 0(ra)` | 从ra寄存器中获取返回地址，并返回，常用于子函数返回 |
| `call func` | `auipc ra, offset[31:12] + offset[11]` <br> `jalr ra, offset[11:0](ra)` | 调用子函数func，返回地址保存到ra寄存器中 |
| `tail func` | `auipc x6, offset[31:12] + offset[11]` <br> `jalr x0, offset[11:0](x6)`| 调用子函数func，不保存返回地址 |

## 条件跳转指令

条件跳转指令都采用B类型的指令编码

| 指令 | 指令格式 | 说明 |
|------|----------|------|
| `beq` | `beq rs1, rs2, label` | 相等分支指令 <br> 如果rs1寄存器和rs2寄存器的值相等，则跳转到label处 |
| `bne` | `bne rs1, rs2, label` | 不相等分支指令 <br> 如果rs1寄存器和rs2寄存器的值不相等，则跳转到label处 |
| `blt` | `blt rs1, rs2, label` | 有符号数小于分支指令 <br> 如果rs1寄存器的值小于rs2寄存器的值，则跳转到label处 |
| `bltu` | `bltu rs1, rs2, label` | 无符号数小于分支指令 <br> 与blt指令类似，但rs1寄存器的值和rs2寄存器的值为无符号数 |
| `bgt` | `bgt rs1, rs2, label` | 有符号数大于分支指令 <br> 如果rs1寄存器的值大于rs2寄存器的值，则跳转到label处 |
| `bgtu` | `bgtu rs1, rs2, label` | 无符号数大于分支指令 <br> 与bgt指令类似，但rs1寄存器的值和rs2寄存器的值为无符号数 |
| `bge` | `bge rs1, rs2, label` | 有符号数大于等于分支指令 <br> 如果rs1寄存器的值大于或等于rs2寄存器的值，则跳转到label处 |
| `bgeu` | `bgeu rs1, rs2, label` | 无符号数大于等于分支指令 <br> 与bge指令类似，但rs1寄存器的值和rs2寄存器的值为无符号数 |

![条件跳转指令](/doc/img/chapter3/img_7.png)

操作数offset表示label的地址基于当前PC地址的偏移量。操作数offset是13位有符号立即数。其中，offset[12:1]由指令编码的Bit[31:25]以及Bit[11:7]共同构成，offset[0]默认为0, offset默认是2的倍数，它的最大寻址范围是−4 KB～4 KB，因此上述指令只能跳转到当前PC地址±4KB的范围。若跳转地址大于上述范围，编译器不会报错，因为链接器在链接重定位时会做链接器松弛优化，选择合适的跳转指令。

| 伪指令 | 指令组合 | 判断条件 |
|--------|----------|----------|
| `beqz rs, label` | `beq rs, x0, label` | rs == 0 |
| `bnez rs, label` | `bne rs, x0, label` | rs != 0 |
| `blez rs, label` | `bge x0, rs, label` | rs <= 0 |
| `bgez rs, label` | `bge rs, x0, label` | rs >= 0 |
| `bltz rs, label` | `blt rs, x0, label` | rs < 0 |
| `bgtz rs, label` | `blt x0, rs, label` | rs > 0 |
| `bgt rs, rt, label` | `blt rt, rs, label` | rs > rt |
| `ble rs, rt, label` | `bge rt, rs, label` | rs <= rt |
| `bgtu rs, rt, label` | `bltu rt, rs, label` | rs > rt (无符号数比较) |
| `bleu rs, rt, label` | `bgeu rt, rs, label` | rs <= rt (无符号数比较) |

## CSR指令

![CSR指令](/doc/img/chapter3/img_7.png)

| CSR指令 | 指令格式 | 说明 |
|---------|----------|------|
| `csrrw` | `csrrw rd, csr, rs1` | 原子地交换CSR和rs1寄存器的值 <br> 读取CSR的旧值，将其零扩展到64位，然后写入rd寄存器中，与此同时，rs1寄存器的旧值写入CSR中 |
| `csrrs` | `csrrs rd, csr, rs1` | 原子地读取CSR的值并设置CSR中相应的位 <br> 读取CSR的旧值，将其零扩展到64位，然后写入rd寄存器中，与此同时，以rs1寄存器的值为掩码，设置CSR相应的位 |
| `csrrc` | `csrrc rd, csr, rs1` | 原子地读取CSR的值并清除CSR中相应的位 <br> 读取CSR的旧值，将其零扩展到64位，然后写入rd寄存器中，与此同时，以rs1寄存器的值为掩码，清除CSR中相应的位 |
| `csrrwi` | `csrrwi rd, csr, uimm` | 与csrrw指令类似，区别在于使用5位无符号立即数代替rs1 |
| `csrrsi` | `csrrsi rd, csr, uimm` | 与csrrs指令类似，区别在于使用5位无符号立即数代替rs1 |
| `csrrci` | `csrrci rd, csr, uimm` | 与csrrc指令类似，区别在于使用5位无符号立即数代替rs1 |

| 伪指令 | 指令组合 | 说明 |
|--------|----------|------|
| `csrr rd, csr` | `csrrs rd, csr, x0` | 读取CSR的值 <br> 从控制状态寄存器中读取值到目标寄存器，不改变CSR的内容 |
| `csrw csr, rs` | `csrrw x0, csr, rs` | 写CSR的值 <br> 将寄存器的值写入控制状态寄存器，不保存旧值 |
| `csrs csr, rs` | `csrrsi x0, csr, rs` | 设置CSR的字段 (csr |= rs) <br> 使用寄存器值作为位掩码，设置CSR中的特定位 |
| `csrc csr, rs` | `csrrci x0, csr, rs` | 清除CSR的字段 (csr &= ~rs) <br> 使用寄存器值作为位掩码，清除CSR中的特定位 |
| `csrwi csr, imm` | `csrrwi x0, csr, imm` | 把立即数写入CSR中 |
| `csrsi csr, imm` | `csrrsi x0, csr, imm` | 设置CSR的字段 (csr |= imm) |
| `csrci csr, imm` | `csrrci x0, csr, imm` | 清除CSR的字段 (csr &= ~imm) |

## 寻址范围

RISC-V支持长距离寻址和短距离寻址。
+ 长距离寻址：通过AUIPC指令可以实现基于当前PC偏移量±2 GB范围的寻址，这种寻址方式叫作PC相对寻址，不过AUIPC指令只能寻址到与4KB对齐的地方。
+ 短距离寻址：有些指令（如ADDI指令、加载和存储指令等）可以实现基于基地址短距离寻址，即寻址范围为−2048～2047，这个范围正好是4KB大小内部的寻址范围。

长距离寻址和短距离寻址结合可以实现基于当前PC偏移量±2 GB范围内任意地址的寻址。
对于跳转指令来说，RISC-V也支持长跳转模式和短跳转模式，这些模式在链接器松弛优化时会用到。
+ 长跳转模式：通过AUIPC与JALR指令实现基于当前PC偏移量±2 GB范围的跳转。
+ 短跳转模式：JAL指令可以实现基于当前PC偏移量±1 MB范围的跳转。

> 注：陷阱：为什么调用RET指令之后就进入死循环
> 使用CALL指令跳转到子函数时会修改ra寄存器（返回地址）的值，把当前PC+4写入ra寄存器中。这就把父函数的返回地址给修改了，导致父函数调用RET指令返回时崩溃，进入死循环。

解决办法是在遇到嵌套调用函数时在父函数里把`ra`寄存器的值保存到临时寄存器中。在父函数调用RET指令返回之前，先从临时寄存器中恢复`ra`寄存器的值，再调用RET指令以返回。

## 参考链接

+ [https://doc.rust-lang.org/reference/inline-assembly.html](https://doc.rust-lang.org/reference/inline-assembly.html)
+ [https://michaeljclark.github.io/asm.html](https://michaeljclark.github.io/asm.html)