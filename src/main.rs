#![no_main]
#![no_std]
#![feature(stmt_expr_attributes)]

mod console;
mod lazy_items;
mod unit_3;

unsafe fn rust_entry(hartid: usize, dtb: usize) {
    unit_3::lab_1();
    unit_3::lab_2();
    println!("Hello, world!");
    println!("hartid = {}, dtb = {:#x}", hartid, dtb);
}
