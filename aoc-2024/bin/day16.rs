use aoc_2024::day16;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 16: DISPLAY NAME")
        .solution(|iter| day16::get_answer(iter))
        .run("inputs/day16.txt");
}
