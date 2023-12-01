use aoc_2023::day1;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 1: Trebuchet?!")
        .solution(|iter| day1::calibrate(iter))
        .run("inputs/day1.txt");
}
