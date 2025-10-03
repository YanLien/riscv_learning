#![no_main]
#![no_std]
#![feature(stmt_expr_attributes)]

mod console;
mod lazy_items;

unsafe fn rust_entry(hartid: usize, dtb: usize) {
    core::arch::asm!("
        addi x1, x2, -2048
    ");
    println!("Hello, world!");
    println!("hartid = {}, dtb = {:#x}", hartid, dtb);
}
