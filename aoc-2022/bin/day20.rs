use aoc_2022::day20;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 20: DISPLAY NAME")
        .solution(|iter| day20::get_answer(iter))
        .run("inputs/day20.txt");
}
