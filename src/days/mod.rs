use crate::print;
use crate::println;
mod day01;
mod day02;

const VTABLE: &[VTableEntry] = &[
    VTableEntry::new(1, 0, day01::part1),
    VTableEntry::new(1, 1, day01::part2),
    VTableEntry::new(2, 0, day02::part1),
    VTableEntry::new(2, 1, day02::part2),
];
struct VTableEntry {
    pub day: usize,
    pub part: usize,
    pub f: fn(&str),
}
pub fn run(day: usize, part: usize, input: &str) {
    let Some(entry) = VTABLE
        .iter()
        .find(|entry| entry.day == day && entry.part == part)
    else {
        println!("day ", day, " and part ", part, " was not registered");
        return;
    };
    (entry.f)(input)
}
impl VTableEntry {
    pub const fn new(day: usize, part: usize, f: fn(&str)) -> Self {
        Self { day, part, f }
    }
}
