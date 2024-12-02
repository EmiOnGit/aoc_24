use arrayvec::ArrayVec;

use crate::{print, println};

pub fn part1(input: &str) {
    let sum = input
        .lines()
        .map(line_to_report)
        .map(is_report_valid)
        .filter(Result::valid)
        .count();
    println!("total sum is ", sum);
}
fn line_to_report(input: &str) -> ArrayVec<i32, 128> {
    input
        .split_whitespace()
        .map(str::parse::<i32>)
        .flatten()
        .collect()
}
pub fn part2(input: &str) {
    let mut sum = 0;

    for report in input.lines().map(line_to_report) {
        let result = is_report_valid(report.clone());
        if result.valid() {
            sum += 1;
        } else {
            let any_valid_route = (0..report.len())
                .into_iter()
                .map(|i| {
                    let mut report = report.clone();
                    report.remove(i);
                    report
                })
                .map(is_report_valid)
                .any(|arg0: Result| Result::valid(&arg0));
            if any_valid_route {
                sum += 1;
            }
        }
    }
    println!("total sum is ", sum);
}
enum Dir {
    Inc,
    Dec,
    Unknown,
}
#[derive(PartialEq)]
enum Result {
    Valid,
    Invalid,
}
impl Result {
    pub fn valid(&self) -> bool {
        *self == Result::Valid
    }
}
fn is_report_valid(levels: impl IntoIterator<Item = i32>) -> Result {
    let mut levels = levels.into_iter();
    let Some(mut prev_level) = levels.next() else {
        return Result::Invalid;
    };
    let mut dir = Dir::Unknown;
    for level in levels {
        let difference = level - prev_level;
        if difference.abs() > 3 || difference == 0 {
            return Result::Invalid;
        }
        match dir {
            Dir::Inc => {
                if difference < 0 {
                    return Result::Invalid;
                }
            }
            Dir::Dec => {
                if difference > 0 {
                    return Result::Invalid;
                }
            }
            Dir::Unknown => {
                if difference > 0 {
                    dir = Dir::Inc;
                } else {
                    dir = Dir::Dec
                }
            }
        }
        prev_level = level;
    }
    return Result::Valid;
}
