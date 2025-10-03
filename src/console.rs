use core::fmt::{Error, Write};

pub fn putchar(c: u8) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c as usize);
}

pub fn write_str(s: &str) {
    for &b in s.as_bytes() {
        putchar(b);
    }
}

pub fn getchar() -> u8 {
    #[allow(deprecated)]
    sbi_rt::legacy::console_getchar() as u8
}

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        write_str(s);
        Ok(())
    }
}

pub fn __print_impl(args: core::fmt::Arguments) {
    Console.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        $crate::console::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::__print_impl(format_args!($($arg)*));
    }
}
