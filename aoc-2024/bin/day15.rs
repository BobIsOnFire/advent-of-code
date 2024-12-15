use aoc_2024::day15;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 15: DISPLAY NAME")
        .solution(|iter| day15::get_answer(iter))
        .run("inputs/day15.txt");
}
