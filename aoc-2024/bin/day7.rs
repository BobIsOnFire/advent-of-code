use aoc_2024::day7;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 7: DISPLAY NAME")
        .solution(|iter| day7::get_answer(iter))
        .run("inputs/day7.txt");
}
