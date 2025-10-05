use core::ptr::read_volatile;

use crate::println;

pub fn lab_1() {
    let x: u64;
    let y: u64;

    unsafe {
        core::arch::asm!(
            "   ",
            "lui t1, 0x400",
            out("t0") x,
            out("t1") y,
        );
    }
    println!("x = {:#x}, y = {:#x}", x, y);

    let z: usize;
    unsafe {
        core::arch::asm!(
            "lui a0, 0x8200f",
            out("a0") z,
        )
    }

    println!("z = {:#x}", z);

    let u: usize;
    unsafe {
        core::arch::asm!(
            "mv a1, x0",  // a1 = x0 = 0
            "addi a1, a1, 16", // a1 = a1 + 16
            out("a1") u,
        )
    }

    println!("u = {:#x}", u);

    let v1: u64;
    let v2: u64;

    unsafe {
        core::arch::asm!(
            "lui {tmp}, 0x80200",       // tmp = 0xFFFFFFFF80200000
            "slli {tmp}, {tmp}, 32",    // 左移32位
            "srli {tmp}, {tmp}, 32",    // 逻辑右移（零扩展）= 0x0000000080200000
            "lwu {result1}, 0({tmp})",   // 从正确地址读取并零扩展
            "ld {result2}, 0({tmp})",    // 从正确地址读取
            tmp = out(reg) _,
            result1 = lateout(reg) v1,
            result2 = lateout(reg) v2,
        );
    }

    println!("v1 = {:#x}, v2 = {:#x}", v1, v2);

    let v: u32;
    unsafe {
        let addr = 0x80200000 as *const u32;
        v = read_volatile(addr);
    }
    println!("v = {:#x}", v);

    let v: u64;
    unsafe {
        let addr = 0x80200000 as *const u64;
        v = read_volatile(addr);
    }
    println!("v = {:#x}", v);
}

pub fn lab_2() {
    let t0: usize;
    let t1: usize;
    const MY_OFFSET: isize = -2048;
    unsafe {
        core::arch::asm!(
            "auipc t0, 1",
            "addi t0, t0, {MY_OFFSET}",
            "ld t1, {MY_OFFSET}(t0)",
            MY_OFFSET = const MY_OFFSET,
            out("t0") t0,
            out("t1") t1,
        );
    }

    println!("t0 = {:#x}, t1 = {:#x}", t0, t1);
}
