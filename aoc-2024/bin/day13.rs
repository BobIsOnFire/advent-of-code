use aoc_2024::day13;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 13: DISPLAY NAME")
        .solution(|iter| day13::get_answer(iter))
        .run("inputs/day13.txt");
}
