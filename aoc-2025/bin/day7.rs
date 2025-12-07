use aoc_2025::day7;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 7: Laboratories")
        .solution(|iter| day7::count_beams(iter))
        .run("inputs/day7.txt");
}
