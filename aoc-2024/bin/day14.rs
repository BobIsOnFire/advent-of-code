use aoc_2024::day14;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 14: DISPLAY NAME")
        .solution(|iter| day14::get_answer(iter))
        .run("inputs/day14.txt");
}
