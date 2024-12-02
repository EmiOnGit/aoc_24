#![no_std]
pub mod days;
pub mod io;
pub mod print;
pub mod syscall;
pub mod prelude {

    pub use crate::syscall;
}
