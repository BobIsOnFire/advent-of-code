use aoc_2024::day1;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 1: Historian Hysteria")
        .solution(|iter| day1::get_distance(iter))
        .run("inputs/day1.txt");
}
