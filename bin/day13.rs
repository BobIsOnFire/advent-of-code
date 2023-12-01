use aoc2022::day13;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 13: Distress Signal")
        .solution(|iter| day13::determine_order(iter))
        .run("inputs/day13.txt");
}
