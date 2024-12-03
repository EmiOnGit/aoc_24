use crate::{print, println};
pub fn part1(input: &str) {
    let sum: usize = process_substring(input);
    println!("total sum is ", sum);
}
/// Expects to have instr begin at the first number of the mul instruction
/// For example if the input string is "asd_mul(1,2)ds", this function returns Some(2) for the instr string "1,2)ds"
pub fn process_mul_instr(instr: &str) -> Option<usize> {
    let (first_n, rest) = instr.split_once(",")?;
    let first_n = first_n.parse::<usize>().ok()?;
    let (sec_n, _rest) = rest.split_once(")")?;
    let sec_n = sec_n.parse::<usize>().ok()?;

    Some(first_n * sec_n)
}
pub fn part2(input: &str) {
    let mut parts = input.split("don't()");
    // The first part doesn't have the "do" instr but should be counted.
    // Therefore it get's processed regardless
    let mut sum = process_substring(parts.next().unwrap());
    for section in parts {
        let Some((_, do_section)) = section.split_once("do()") else {
            continue;
        };
        sum += process_substring(do_section);
    }
    println!("total sum is ", sum);
}
pub fn process_substring(input: &str) -> usize {
    input.split("mul(").filter_map(process_mul_instr).sum()
}
