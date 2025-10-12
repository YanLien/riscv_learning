use crate::{println, rust_entry};
use core::{arch::naked_asm, panic::PanicInfo};

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("
        la a3, _sbss
        la a4, _ebss
        bge a3, a4, 2f      # 如果 _sbss >= _ebss，跳过清零
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

const LOGO: &str = r#"
db    db  .d8b.  d8b   db db      d888888b d88888b d8b   db 
`8b  d8' d8' `8b 888o  88 88        `88'   88'     888o  88 
 `8bd8'  88ooo88 88V8o 88 88         88    88ooooo 88V8o 88 
   88    88~~~88 88 V8o88 88         88    88~~~~~ 88 V8o88 
   88    88   88 88  V888 88booo.   .88.   88.     88  V888 
   YP    YP   YP VP   V8P Y88888P Y888888P Y88888P VP   V8P 
"#;

pub fn print_logo() {
    println!();
    println!("{}", LOGO);
    println!();
}
