use crate::{print, println};
pub fn part1(input: &str) {
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == 'X' {
                sum += check_xmas(input, x, y)
            }
        }
    }
    println!("sum ", sum);
}

pub fn part2(input: &str) {
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == 'A' {
                if check_cross_mas(input, x, y) {
                    sum += 1;
                }
            }
        }
    }
    println!("sum ", sum);
}

fn check_cross_mas(input: &str, x: usize, y: usize) -> bool {
    let mas = "MAS";
    let deltas = [(1isize, 1isize), (-1, 1), (-1, -1), (1, -1)];
    let mut correct_lines = 0;
    'delta: for delta in deltas {
        for (i, c) in mas.char_indices() {
            let delta2 = (delta.0 * (i as isize - 1), delta.1 * (i as isize - 1));
            let Some(x) = x.checked_add_signed(delta2.0) else {
                continue 'delta;
            };
            let Some(y) = y.checked_add_signed(delta2.1) else {
                continue 'delta;
            };
            if !is_char(input, x, y, c) {
                continue 'delta;
            }
        }
        correct_lines += 1;
    }
    correct_lines == 2
}
fn check_xmas(input: &str, x: usize, y: usize) -> usize {
    let xmas = "XMAS";
    let deltas = [
        (1isize, 0isize),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut sum = 0;
    'delta: for delta in deltas {
        for (i, c) in xmas.char_indices() {
            let delta2 = (delta.0 * i as isize, delta.1 * i as isize);
            let Some(x) = x.checked_add_signed(delta2.0) else {
                continue 'delta;
            };
            let Some(y) = y.checked_add_signed(delta2.1) else {
                continue 'delta;
            };
            if !is_char(input, x, y, c) {
                continue 'delta;
            }
        }
        sum += 1;
    }
    return sum;
}
fn is_char(input: &str, x: usize, y: usize, c: char) -> bool {
    let Some(line) = input.lines().nth(y) else {
        return false;
    };
    let Some(at_pos) = line.chars().nth(x) else {
        return false;
    };
    at_pos == c
}
