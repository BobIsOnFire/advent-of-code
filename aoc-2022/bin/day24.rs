use aoc_2022::day24;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 24: Blizzard Basin")
        .solution(|iter| day24::count_path_minutes(iter))
        .run("inputs/day24.txt");
}
