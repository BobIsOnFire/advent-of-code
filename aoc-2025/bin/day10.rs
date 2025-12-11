use aoc_2025::day10;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 10: DISPLAY NAME")
        .solution(|iter| day10::get_answer(iter))
        .run("inputs/day10.txt");
}
