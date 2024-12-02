use core::arch::asm;

use crate::{print, println};

pub unsafe fn exit(code: i32) -> ! {
    let syscall_number: u64 = 60;
    asm!(
        "syscall",
        in("rax") syscall_number,
        in("rdi") code,
        options(noreturn)
    )
}
#[derive(Clone, Copy)]
pub enum FileDesc {
    In,
    Out,
    Err,
    Other(usize),
}
impl From<FileDesc> for usize {
    fn from(value: FileDesc) -> Self {
        match value {
            FileDesc::In => 0,
            FileDesc::Out => 1,
            FileDesc::Err => 2,
            FileDesc::Other(i) => i,
        }
    }
}
pub fn read(fd: FileDesc, buf: &mut [u8]) -> usize {
    unsafe { syscall3(0, usize::from(fd), buf.as_ptr() as usize, buf.len()) }
}
pub fn open(path: *const u8, flags: usize) -> Option<FileDesc> {
    let ret = unsafe { syscall3(2, path as usize, flags, 0) };
    // let ret = raw::sys_open(path, flags);
    let reti32 = isize::from_be_bytes(ret.to_be_bytes());
    if reti32 <= 0 {
        println!("error while opening file with error", -reti32 as usize);
        None
    } else {
        println!("opened file with: fd: ", reti32 as usize);
        Some(FileDesc::Other(reti32 as usize))
    }
}
pub unsafe fn sys_write(fd: FileDesc, buf: *const u8, count: usize) -> usize {
    syscall3(1, usize::from(fd), buf as usize, count)
}
#[inline]
/// syscall with 3 args
unsafe fn syscall3(n: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rcx") _, // rcx is used to store old rip
        lateout("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}
pub unsafe fn strlen(mut s: *const u8) -> usize {
    let mut count = 0;
    while *s != b'\0' {
        count += 1;
        s = s.add(1);
    }
    count
}
