#![no_std]
// pub mod io;
pub mod days;
pub mod io;
pub mod print;
pub mod syscall;
pub mod prelude {

    pub use crate::syscall;
}
#[macro_export]
macro_rules! run {
    ($($arg:expr),+) => {
        print!($($arg),+,"\n");
    };
}
