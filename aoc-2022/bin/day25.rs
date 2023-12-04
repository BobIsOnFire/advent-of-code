use aoc_2022::day25;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 25: Full of Hot Air")
        .solution(|iter| day25::translate_numbers(iter))
        .run("inputs/day25.txt");
}
