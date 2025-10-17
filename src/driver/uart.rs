/*
设备类型：NS16550A (UART)
设备名称：uart0
内存映射地址：0x10000000
中断号：10
 */

use core::ptr::NonNull;

pub struct NS16550A {
    base_addr: NonNull<u8>,
}

impl NS16550A {
    pub fn new(base_addr: NonNull<u8>) -> Self {
        NS16550A { base_addr }
    }

    pub fn write_reg(&self, offset: usize, value: u8) {
        todo!()
    }

    pub fn read_reg(&self, offset: usize) -> u8 {
        todo!()
    }

    pub fn init(&self) -> Result<(), &'static str> {
        todo!()
    }
    
    pub fn write_byte(&self, byte: u8) {
        todo!()
    }

    pub fn read_byte(&self) -> u8 {
        todo!()
    }
}