use aoc_2024::day18;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 18: DISPLAY NAME")
        .solution(|iter| day18::get_answer(iter))
        .run("inputs/day18.txt");
}
