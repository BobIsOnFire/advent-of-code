use aoc_2024::day11;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 11: DISPLAY NAME")
        .solution(|iter| day11::get_answer(iter))
        .run("inputs/day11.txt");
}
