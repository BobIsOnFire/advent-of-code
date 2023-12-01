use aoc_2022::day14;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 14: Regolith Reservoir")
        .solution(|iter| day14::count_stable_units(iter))
        .run("inputs/day14.txt");
}
