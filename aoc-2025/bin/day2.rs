use aoc_2025::day2;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 2: Gift Shop")
        .solution(|iter| day2::find_invalid_numbers(iter))
        .run("inputs/day2.txt");
}
