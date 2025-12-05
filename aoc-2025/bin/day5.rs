use aoc_2025::day5;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 5: DISPLAY NAME")
        .solution(|iter| day5::get_answer(iter))
        .run("inputs/day5.txt");
}
