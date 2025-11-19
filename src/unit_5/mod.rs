pub mod sym;

use core::{arch::global_asm, num};
use crate::println;
use sym::*;

unsafe extern "C" {
    fn find_max(values: *const i32, size: i32) -> i32;
    fn find_two_max(value1: i32, value2: i32) -> i32;
    fn compare_num(num1: i32, num2: i32) -> i32;
    fn macro_test() -> i32;
}

global_asm!(include_str!("test.S"));
// global_asm!(include_str!("macro.S"));

pub fn unit_test() {
    println!("unit 5 test");

    let numbers: [i32; 10] = [15, 43, 35, 6, 7, 8, 8, 9, 90, 96];
    let max_value = unsafe { find_max(numbers.as_ptr(), 10) };

    println!("max_value: {}", max_value);

    let max_value = unsafe { find_two_max(43, 57) };

    println!("max_value: {}", max_value);

    let max_value = unsafe { compare_num(67, 54) };

    println!("max_value: {}", max_value);

    let symbols = SymbolTable::new();

    if let Some(addr) = symbols.find("add_numbers") {
        println!("Found add_numbers at: 0x{:x}", addr);

        let func: extern "C" fn(i32, i32) -> i32 =
            unsafe { core::mem::transmute(addr as *const ()) };
        let result = func(10, 20);
        println!("Result: {}", result);
    }

    let x = unsafe{ macro_test() };

    println!("{}", x);


    println!("unit 5 end");
}

#[unsafe(no_mangle)]
pub extern "C" fn compare_number(num1: i32, num2: i32) -> i32 {
    if num1 >= num2 {
        return num1;
    }
    num2
}

#[unsafe(no_mangle)]
pub extern "C"  fn add_1(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C"  fn add_2(a: i32, b: i32) -> i32 {
    2 * a + b
}
