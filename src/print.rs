use crate::syscall::{sys_write, FileDesc};

fn print_str(buf: &[u8]) {
    unsafe {
        sys_write(FileDesc::Out, buf.as_ptr(), buf.len());
    }
}
pub fn print_intern(buffer: &[Printable]) {
    for token in buffer {
        token.print();
    }
}
pub enum Printable<'a> {
    Text(&'a [u8]),
    Number(usize),
}
impl From<usize> for Printable<'_> {
    fn from(value: usize) -> Self {
        Printable::Number(value)
    }
}
impl<'a> From<&'a str> for Printable<'a> {
    fn from(value: &'a str) -> Self {
        Printable::Text(value.as_bytes())
    }
}
impl<'a> From<&'a [u8]> for Printable<'a> {
    fn from(value: &'a [u8]) -> Self {
        Printable::Text(value)
    }
}
impl<'a> Printable<'a> {
    pub(crate) fn print(&self) {
        match self {
            Printable::Text(s) => print_str(s),
            Printable::Number(n) => {
                if *n >= 10 {
                    Printable::Number(n / 10).print();
                }
                let c = b'0' + (n % 10) as u8;
                print_str(&[c]);
            }
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:expr),+) => {
        crate::print::print_intern(&[
            $($arg.into()),+
        ])
    };
}
#[macro_export]
macro_rules! println {
    ($($arg:expr),+) => {
        print!($($arg),+,"\n");
    };
}
