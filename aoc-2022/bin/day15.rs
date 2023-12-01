use aoc_2022::day15;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 15: Beacon Exclusion Zone")
        .solution(|iter| day15::find_missing_beacon(iter))
        .run("inputs/day15.txt");
}
