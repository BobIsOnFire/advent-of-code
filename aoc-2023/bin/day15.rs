use aoc_2023::day15;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 15: Lens Library")
        .solution(|iter| day15::focus_lenses(iter))
        .run("inputs/day15.txt");
}
