use aoc_2024::day24;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 24: DISPLAY NAME")
        .solution(|iter| day24::get_answer(iter))
        .run("inputs/day24.txt");
}
