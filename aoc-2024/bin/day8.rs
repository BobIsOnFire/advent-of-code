use aoc_2024::day8;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 8: DISPLAY NAME")
        .solution(|iter| day8::get_answer(iter))
        .run("inputs/day8.txt");
}
