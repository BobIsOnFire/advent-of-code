use aoc_2024::day22;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 22: DISPLAY NAME")
        .solution(|iter| day22::get_answer(iter))
        .run("inputs/day22.txt");
}
