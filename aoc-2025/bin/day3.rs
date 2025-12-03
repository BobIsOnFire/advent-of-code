use aoc_2025::day3;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 3: Lobby")
        .solution(|iter| day3::get_total_joltage(iter))
        .run("inputs/day3.txt");
}
