use core::cmp::Ordering;

use arrayvec::ArrayVec;

use crate::{print, println};
pub fn part1(input: &str) {
    let rules = Rules::<2048>::new(input);
    let sum: usize = input
        .lines()
        .filter(|line| line.contains(','))
        .map(parse_page::<32>)
        .filter_map(|line| {
            for i in 0..line.len() {
                for j in i..line.len() {
                    if rules.check(line[i], line[j]) == Ordering::Greater {
                        return None;
                    }
                }
            }
            return Some(line[line.len() / 2]);
        })
        .sum();
    println!("sum ", sum);
}
pub fn part2(input: &str) {
    let rules = Rules::<2048>::new(input);
    let sum: usize = input
        .lines()
        .filter(|line| line.contains(','))
        .map(parse_page::<32>)
        .filter_map(|line| {
            let mut line2 = line.clone();
            // NOTE
            // use a stable sort on Rules::check for this part.
            // Couldn't get it to work because of linking errors so I used this.
            sort_middle(&rules, line2.as_mut_slice());
            (line2 != line).then_some(line2[line2.len() / 2])
        })
        .sum();
    println!("sum ", sum);
}
fn parse_page<const N: usize>(line: &str) -> ArrayVec<usize, N> {
    line.split(',')
        .filter_map(|token| token.parse::<usize>().ok())
        .collect::<ArrayVec<usize, N>>()
}
fn sort_middle<const N: usize>(rules: &Rules<N>, ar: &mut [usize]) {
    let middle = ar.len() / 2;
    for i in 0..middle {
        if rules.check(ar[i], ar[middle]) == Ordering::Greater {
            ar.swap(i, middle);
            sort_middle(rules, ar);
        }
    }
    for i in (middle + 1)..ar.len() {
        if rules.check(ar[i], ar[middle]) == Ordering::Less {
            ar.swap(i, middle);
            sort_middle(rules, ar);
        }
    }
}
struct Rules<const N: usize> {
    before: ArrayVec<usize, N>,
    after: ArrayVec<usize, N>,
}
impl<const N: usize> Rules<N> {
    pub fn check(&self, before: usize, after: usize) -> Ordering {
        for i in 0..self.before.len() {
            if self.before[i] == after && self.after[i] == before {
                return Ordering::Greater;
            }
            if self.before[i] == before && self.after[i] == after {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
    pub fn new(input: &str) -> Self {
        let (before, after): (ArrayVec<usize, N>, ArrayVec<usize, N>) = input
            .lines()
            // every line without a '|' get's filtered here
            .filter_map(|line| line.split_once('|'))
            .map(|(bef, aft)| (bef.parse::<usize>().unwrap(), aft.parse::<usize>().unwrap()))
            .unzip();
        return Rules { before, after };
    }
}
