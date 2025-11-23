# 虚拟化扩展

本章思考题
1. 实现虚拟化的3个要素是什么？
2. 什么是GVA、GPA、HVA和HPA？
3. RISC-V在CPU虚拟化中做了哪些改进？
4. 处于HS模式下的处理器能访问哪些系统寄存器？
5. 请简述虚拟化场景中两阶段地址映射的过程。
6. RISC-V为虚拟化新增的HLV/HSV指令有什么用途？
7. 在RISC-V虚拟化扩展中，VMM如何进入虚拟机？
8. 在RISC-V虚拟化扩展中，VM有哪些途径可以陷入VMM？
9. 在RISC-V虚拟化扩展中，如何把一个中断注入VM？
10. 在虚拟化场景中，什么是“陷入与模拟”机制？

**平台虚拟化(platform virtualization)：针对计算机和操作系统的虚拟化，如KVM等。**

**资源虚拟化(resource virtualization)：针对特定系统资源的虚拟化，包括内存、存储器、网络资源等，如容器技术。**

**应用程序虚拟化(application virtualization)：针对应用程序的虚拟化，包括仿真、模拟、解释技术等，如Java虚拟机。**

## 虚拟化技术介绍

虚拟化的主要思想是利用虚拟机监控程序(Virtual MachineMonitor, VMM)在同一物理硬件上创建多个虚拟机，这些虚拟机运行时就像真实的物理机器一样。虚拟机监控程序又被称为虚拟机管理程序(hypervisor)。

### 虚拟化技术的发展历史

+ 资源控制(resource control)。VMM必须能够管理所有的系统资源。
+ 等价性(equivalence)。虚拟机的运行行为与裸机的运行行为一致。
+ 效率性(efficiency)。虚拟机运行的程序不受VMM干涉。

