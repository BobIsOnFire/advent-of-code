use aoc_2024::day12;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 12: DISPLAY NAME")
        .solution(|iter| day12::get_answer(iter))
        .run("inputs/day12.txt");
}
