use crate::rust_entry;
use core::{arch::naked_asm, panic::PanicInfo};

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("
        la a3, _sbss
        la a4, _ebss
        ble a4, a3, 2f
    1:
        sd zero, (a3)
        add a3, a3, 8
        blt a3, a4, 1b
    2:
        la      sp, boot_stack_top      // setup boot stack

        la      a2, {entry}
        jalr    a2                      // call rust_entry(hartid, dtb)
        j       .",
        entry = sym rust_entry,
    )
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
