use aoc_2023::day9;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 9: Mirage Maintenance")
        .solution(|iter| day9::extrapolate_sequence(iter))
        .run("inputs/day9.txt");
}
