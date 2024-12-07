use crate::print;
use crate::println;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

const VTABLE: &[VTableEntry] = &[
    VTableEntry::new(1, 0, day01::part1),
    VTableEntry::new(1, 1, day01::part2),
    VTableEntry::new(2, 0, day02::part1),
    VTableEntry::new(2, 1, day02::part2),
    VTableEntry::new(3, 0, day03::part1),
    VTableEntry::new(3, 1, day03::part2),
    VTableEntry::new(4, 0, day04::part1),
    VTableEntry::new(4, 1, day04::part2),
    VTableEntry::new(5, 0, day05::part1),
    VTableEntry::new(5, 1, day05::part2),
    VTableEntry::new(6, 0, day06::part1),
    VTableEntry::new(6, 1, day06::part2),
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
