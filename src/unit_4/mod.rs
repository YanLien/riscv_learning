use core::arch::asm;
use crate::println;

/// Core backtrace function - traverses the stack frame chain
pub fn walk_stack() {
    // 1. Get current register state
    let fp = get_fp();
    let sp = get_sp();
    let ra = get_ra();
    
    println!("=== Current Function State ===");
    println!("FP (s0): 0x{:016x}", fp);
    println!("SP:      0x{:016x}", sp);
    println!("RA:      0x{:016x}", ra);
    println!("Current stack frame size: {} bytes\n", fp - sp);
    
    // 2. Traverse the stack frame chain
    println!("=== Stack Backtrace ===");
		let mut current_fp = fp;
		let mut frame_num = 0;
		let stack_top = sp;
    
    while current_fp != 0 && frame_num < 10 {
        unsafe {
            // RISC-V ABI specification:
            // [FP - 16] stores previous FP
            // [FP - 8]  stores return address RA
            let saved_fp = *((current_fp - 16) as *const usize);
            let saved_ra = *((current_fp - 8) as *const usize);
            
            // Calculate stack frame size
            let frame_size = if saved_fp > current_fp {
                saved_fp - current_fp
            } else {
                0
            };
            
            println!("Frame {}: FP=0x{:x}, RA=0x{:x}, Size={} bytes", 
                     frame_num, current_fp, saved_ra, frame_size);
            
            // Safety check: avoid invalid pointers
            if saved_fp <= current_fp || saved_fp == 0 {
                break;
            }
            
            current_fp = saved_fp;
            frame_num += 1;
        }
    }
    
    // 3. Output stack range and total size
    println!("\n=== Stack Statistics ===");
    // println!("Stack top (SP):  0x{:016x}", stack_top);
    println!("Stack base (FP): 0x{:016x}", fp);
    // println!("Address range: [0x{:x}, 0x{:x}]", stack_top, fp);
    // println!("Total stack usage: {} bytes", fp - stack_top);
    println!("Number of stack frames: {}", frame_num + 1);
}

#[inline(always)]
fn get_fp() -> usize {
    let fp: usize;
    unsafe { asm!("mv {}, s0", out(reg) fp); }
    fp
}

#[inline(always)]
fn get_sp() -> usize {
    let sp: usize;
    unsafe { asm!("mv {}, sp", out(reg) sp); }
    sp
}

#[inline(always)]
fn get_ra() -> usize {
    let ra: usize;
    unsafe { asm!("mv {}, ra", out(reg) ra); }
    ra
}

// 测试代码
#[inline(never)]
fn func2() {
    println!("\n>>> start func2");
    walk_stack();
}

#[inline(never)]
fn func1() {
    println!("\n>>> start func1");
    func2();
}

#[inline(never)]
pub fn unit_test() {
    println!(">>> start unit_test");
    func1();
}
