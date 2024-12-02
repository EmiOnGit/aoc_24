use crate::{print, println};
use arrayvec::ArrayVec;

pub fn part1(input: &str) {
    let (mut v0, mut v1) = get_vecs::<1024>(input);
    v0.sort_unstable();
    v1.sort_unstable();
    let sum: usize = v0
        .into_iter()
        .zip(v1)
        .map(|(x, y)| x.max(y) - x.min(y))
        .sum();
    println!("total sum is ", sum);
}
pub fn part2(input: &str) {
    let (mut v0, mut v1) = get_vecs::<1024>(input);
    v0.sort_unstable();
    v1.sort_unstable();
    let sum: usize = v0
        .into_iter()
        .map(|x| v1.iter().filter(|y| **y == x).count() * x)
        .sum();
    println!("total sum is ", sum);
}
fn get_vecs<const N: usize>(input: &str) -> (ArrayVec<usize, N>, ArrayVec<usize, N>) {
    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(v1, v2)| (v1.trim(), v2.trim()))
        .map(|(v1, v2)| (v1.parse::<usize>().unwrap(), v2.parse::<usize>().unwrap()))
        .unzip()
}
