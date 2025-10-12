use core::arch::global_asm;
use crate::println;


global_asm!(include_str!("load.S"));
global_asm!(include_str!("pc_address.S"));
global_asm!(include_str!("memset.S"));
global_asm!(include_str!("memcpy.S"));
global_asm!(include_str!("jump.S"));
global_asm!(include_str!("sub_jump.S"));

unsafe extern "C" {
    fn load(results: *mut u64);
    fn pc_address() -> (u64, u64);
    fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8;
    fn memcpy(src: *const u8, dst: *mut u8);
    fn compare_and_return(a: u64, b: u64) -> u64;
    fn sel_test(a: u64, b: u64) -> u64;
    fn bl_test(a: u64, b: u64) -> u64;
}

pub fn unit_test() {
    println!("Unit 3: RISCV Base Assembly Language Test");

    let mut results = [0u64; 6];
    unsafe { load(results.as_mut_ptr()) };
    for (i, v) in results.iter().enumerate() {
        println!("x{} = {:#x}", i, v);
    }

    let (t0, t1) = unsafe { pc_address() };
    println!("t0 = {:#x}, t1 = {:#x}", t0, t1);

    let src_addr = 0x80200000 as *const u8;
    let dst_addr = 0x80210000 as *mut u8;
    
    println!("{:#x} {:#x}", unsafe { *src_addr }, unsafe { *dst_addr });

    unsafe { memcpy(src_addr, dst_addr) };

    println!("{:#x} {:#x}", unsafe { *src_addr }, unsafe { *dst_addr });

    let mut str_array = [0u8; 16];
    let result = unsafe { memset(str_array.as_mut_ptr(), 2, str_array.len()) };
    assert_eq!(result as *const u8, str_array.as_ptr());
    println!("memset result ptr: {:?}", str_array);

    let a = 0x1234u64;
    let b = 0x13324u64;
    let c = unsafe { compare_and_return(a, b) };
    println!("compare_and_return({:#x}, {:#x}) = {:#x}", a, b, c);

    let d = unsafe { sel_test(a, b) };
    println!("sel_test({:#x}, {:#x}) = {:#x}", a, b, d);
    let d = unsafe { sel_test(0, b) };
    println!("sel_test({:#x}, {:#x}) = {:#x}", 0, b, d);

    let e = unsafe { bl_test(a, b) };
    println!("bl_test({:#x}, {:#x}) = {:#x}", a, b, e);
    let e = unsafe { bl_test(0, a) };
    println!("bl_test({:#x}, {:#x}) = {:#x}", 0, a, e);

    println!("Unit 3 test finished.");
}