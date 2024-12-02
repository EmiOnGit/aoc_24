use crate::{print, println};

pub fn part1(input: &str) {
    let mut sum = 0;

    'shaft: for line in input.lines() {
        let mut levels = line.split_whitespace().map(str::parse::<i32>).flatten();
        let Some(mut prev_level) = levels.next() else {
            continue 'shaft;
        };
        let mut dir = Dir::Unknown;
        for level in levels {
            let difference = level - prev_level;
            if difference.abs() > 3 || difference == 0 {
                continue 'shaft;
            }
            match dir {
                Dir::Inc => {
                    if difference < 0 {
                        continue 'shaft;
                    }
                }
                Dir::Dec => {
                    if difference > 0 {
                        continue 'shaft;
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
        sum += 1;
    }
    println!("total sum is ", sum);
}
enum Dir {
    Inc,
    Dec,
    Unknown,
}
pub fn part2(input: &str) {
    let mut sum = 0;

    'shaft: for line in input.lines() {
        let levels = line.split_whitespace().map(str::parse::<i32>).flatten();
        'direction: for dir in [Dir::Inc, Dir::Dec] {
            let mut levels = levels.clone();
            let Some(mut prev_level) = levels.next() else {
                continue 'shaft;
            };
            let mut used_dampener = false;
            for level in levels {
                let difference = match dir {
                    Dir::Inc => level - prev_level,
                    Dir::Dec => prev_level - level,
                    Dir::Unknown => panic!(),
                };
                if difference > 3 || difference <= 0 {
                    if used_dampener {
                        continue 'direction;
                    }
                    used_dampener = true;
                    continue;
                }
                prev_level = level;
            }
            sum += 1;
            continue 'shaft;
        }
    }
    println!("total sum is ", sum);
}
