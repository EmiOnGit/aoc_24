use core::slice::from_raw_parts;

use crate::{
    print, println,
    syscall::{self},
};

/// Reads the content of the file to the buffer.
/// Safety:
/// returns silently in case of failure of opening the file without writting the buffer
/// crashes in case of buffer overflow
pub fn read_to_string(buffer: &mut [u8], path: *const u8) {
    let Some(fd) = syscall::open(path, 0) else {
        unsafe {
            let path = from_raw_parts(path, strlen(path));
            println!("failed opening file ", path);
        }
        return;
    };
    let mut read_bytes = 0;
    let buf_len = buffer.len();
    let mut count = 1;
    // return of 0 indicates end of file
    while count != 0 {
        count = syscall::read(fd, &mut buffer[read_bytes..]);
        read_bytes += count;
        if read_bytes >= buf_len {
            println!("buffer for reading the file was to small");
            break;
        }
    }
}
/// Safety:
/// Expects the string to end with '\0'.
/// Will overflow if this is not the case.
pub unsafe fn strlen(mut s: *const u8) -> usize {
    let mut count = 0;
    while *s != b'\0' {
        count += 1;
        s = s.add(1);
    }
    count
}
