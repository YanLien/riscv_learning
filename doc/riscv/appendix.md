# 附录

## RV64I指令速查表

## RV64M指令速查表

## RV64常用伪指令速查表

## RISC-V SBI规范

所有 SBI 函数共享单一的二进制编码，这便于混合使用不同的 SBI 扩展。SBI 规范遵循以下调用约定：

**调用约定**

+ 使用 ECALL 指令作为监督模式和 SEE（监督执行环境）之间的控制转移指令
+ a7 寄存器编码 SBI 扩展 ID（EID）
+ a6 寄存器编码给定扩展 ID 的 SBI 函数 ID（FID），适用于 SBI v0.2 及之后定义的所有扩展
+ 除了 a0 和 a1 之外的所有寄存器必须由被调用者（callee）在 SBI 调用过程中保持不变
+ SBI 函数必须在 a0 和 a1 中返回一对值，其中 a0 返回错误码。这类似于返回以下 C 结构体：

    ``` c
    struct sbiret {
        long error;
        long value;
    };
    ```

**ID 编码规则**

为了兼容性考虑，SBI 扩展 ID（EID）和 SBI 函数 ID（FID）被编码为有符号 32 位整数。当在寄存器中传递时，它们遵循上述标准调用约定规则。

**标准 SBI 错误码**

下表提供了标准 SBI 错误码列表：
表 1. 标准 SBI 错误
| 错误类型 | 值 |
| ------- | --- | 
| SBI_SUCCESS（成功）| 0 | 
| SBI_ERR_FAILED（失败） | -1 | 
| SBI_ERR_NOT_SUPPORTED（不支持） | -2 | 
| SBI_ERR_INVALID_PARAM（无效参数） | -3 |
| SBI_ERR_DENIED（拒绝）| -4 | 
| SBI_ERR_INVALID_ADDRESS（无效地址） | -5 | 
| SBI_ERR_ALREADY_AVAILABLE（已可用） | -6 | 
| SBI_ERR_ALREADY_STARTED（已启动） | -7 | 
| SBI_ERR_ALREADY_STOPPED（已停止）| -8 |

如果 ECALL 使用了不支持的 SBI 扩展 ID（EID）或不支持的 SBI 函数 ID（FID），必须返回错误码 `SBI_ERR_NOT_SUPPORTED`。

**数据类型规范**

每个 SBI 函数应优先使用 unsigned long 作为数据类型。这使得规范保持简单，并易于适配所有 RISC-V ISA 类型。如果数据被定义为 32 位宽，更高特权级软件必须确保只使用 32 位数据。

**Hart 掩码（Hart Mask）**

如果 SBI 函数需要向更高特权模式传递 hart 列表，它必须使用下面定义的 hart 掩码。这适用于 v0.2 及之后定义的所有扩展。
任何需要 hart 掩码的函数都需要传递以下两个参数：

```c
unsigned long hart_mask - 包含 hartid 的标量位向量
unsigned long hart_mask_base - 计算位向量的起始 hartid
```

在单次 SBI 函数调用中，可以设置的最大 hart 数量始终为 XLEN。如果较低特权模式需要传递超过 XLEN 个 hart 的信息，它应该多次调用 SBI 函数。
hart_mask_base 可以设置为 -1，表示可以忽略 hart_mask，并且必须考虑所有可用的 hart。

**Hart 掩码错误**

使用 hart 掩码的任何函数除了特定于函数的错误值外，还可能返回下表 2 中列出的错误值。
表 2. Hart 掩码错误
| 错误码 | 描述 |
| ------ | ---- |
| SBI_ERR_INVALID_PARAM（无效参数）| hart_mask_base 或 hart_mask 中的任何 hartid 无效，即该 hartid 未被平台启用或对监督模式不可用 |

注释：

+ SEE (Supervisor Execution Environment): 监督执行环境
+ Hart: Hardware Thread，硬件线程
+ XLEN: RISC-V 架构中寄存器的位宽（32 位或 64 位）