#![no_main]
#![no_std]
#![feature(stmt_expr_attributes)]

mod console;
mod driver;
mod unit_3;
mod lazy_items;

use crate::{console::shutdown, lazy_items::print_logo};

unsafe fn rust_entry(hartid: usize, dtb: usize) {
    print_logo();

    unit_3::unit_test();
    

    shutdown();
}
