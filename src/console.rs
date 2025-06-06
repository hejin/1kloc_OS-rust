
use core::fmt::{self, Write};
use crate::sbi::sbi_call;

// console putchar using SBI extension 0x01
pub fn _putchar(ch: u8) {
	sbi_call(
		ch as isize, 0, 0, 0, 0, 0,
		0, // function ID
		1  // extension ID: Console putchar
	);
}

struct Console;

impl Write for Console {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for byte in s.bytes() {
			_putchar(byte);
		}
		Ok(())
	}
}

pub fn _printf(args: fmt::Arguments) {
	let _ = Console.write_fmt(args);
}

#[macro_export]
macro_rules! printf {
	($($arg:tt)*) => {
		$crate::console::_printf(core::format_args!($($arg)*));
	};
}

