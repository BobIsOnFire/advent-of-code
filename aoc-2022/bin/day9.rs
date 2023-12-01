use aoc_2022::day9;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 9: Rope Bridge")
        .solution(|iter| day9::count_unique_positions(iter))
        .run("inputs/day9.txt");
}
