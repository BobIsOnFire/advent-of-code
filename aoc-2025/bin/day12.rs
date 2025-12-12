use aoc_2025::day12;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 12: Christmas Tree Farm")
        .solution(|iter| day12::fit_presents(iter))
        .run("inputs/day12.txt");
}
