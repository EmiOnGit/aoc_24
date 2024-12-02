#![no_main]
#![feature(lang_items)]
#![no_std]
#![allow(internal_features)]
#![feature(naked_functions)]
use core::slice::from_raw_parts;
use core::str;
use core::{arch::naked_asm, panic::PanicInfo};

use aoc_24::io::read_to_string;
use aoc_24::{days, prelude::*, print, println};
use syscall::{exit, strlen};
#[link(name = "c")]
extern "C" {}
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _start() -> ! {
    naked_asm!("mov rdi, rsp", "call main")
}
#[no_mangle]
pub unsafe fn main(stack_pointer: *const u8) {
    let argc = *(stack_pointer as *const u64);
    let argv = stack_pointer.add(8) as *const *const u8;

    let (day, part) = read_args(argc, argv);
    let input = read_input_file::<30000>(day);
    let Ok(input) = str::from_utf8(&input) else {
        println!("input was not utf8 encoded");
        syscall::exit(argc as _);
    };
    days::run(day, part, input);
    syscall::exit(argc as _);
}
unsafe fn read_input_file<const BUF_SIZE: usize>(day: usize) -> [u8; BUF_SIZE] {
    let mut buffer = [0u8; BUF_SIZE];
    let mut filename = [0u8; 13];
    filename.copy_from_slice(b"assets/day__\0");
    filename[10] = b'0' + (day / 10) as u8;
    filename[11] = b'0' + (day % 10) as u8;
    read_to_string(&mut buffer, filename.as_ptr());
    return buffer;
}
unsafe fn read_args(argc: u64, argv: *const *const u8) -> (usize, usize) {
    let mut args = from_raw_parts(argv, argc as usize).into_iter();
    // discard program path
    let _ = args.next();
    let Some(day) = args.next() else {
        println!("Run with: cargo r -- {day} [part]");
        syscall::exit(argc as _);
    };
    let day = from_raw_parts(*day, strlen(*day));
    // let day = day;
    let Ok(day) = str::from_utf8(day) else {
        println!("{day} is no valid utf8");
        println!("Run with: cargo r -- {day} [part]");
        syscall::exit(argc as _);
    };
    let Ok(day) = day.parse::<usize>() else {
        println!("{day} is no number");
        syscall::exit(argc as _);
    };
    let Some(part) = args.next() else {
        println!("Run with: cargo r -- {part} [part]");
        syscall::exit(argc as _);
    };
    let part = from_raw_parts(*part, strlen(*part));
    // let part = part;
    let Ok(part) = str::from_utf8(part) else {
        println!("{part} is no valid utf8");
        println!("Run with: cargo r -- {part} [part]");
        syscall::exit(argc as _);
    };
    let Ok(part) = part.parse::<usize>() else {
        println!("{part} is no number");
        syscall::exit(argc as _);
    };
    (day, part)
}
#[panic_handler]
fn panic(msg: &PanicInfo<'_>) -> ! {
    println!("");
    println!("panic with:");
    let text = msg.message().as_str().unwrap_or_default();
    println!(text);
    if let Some(location) = msg.location() {
        let file = location.file();
        let line = location.line() as usize;
        println!("file: ", file, " line: ", line);
    }
    // println!()
    unsafe { exit(1) };
    // loop {}
}

#[lang = "eh_personality"]
fn eh_personality() {}
