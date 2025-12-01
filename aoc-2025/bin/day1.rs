use aoc_2025::day1;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 1: Secret Entrance")
        .solution(|iter| day1::crack_the_safe(iter))
        .run("inputs/day1.txt");
}
