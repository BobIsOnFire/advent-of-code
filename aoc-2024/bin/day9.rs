use aoc_2024::day9;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 9: DISPLAY NAME")
        .solution(|iter| day9::get_answer(iter))
        .run("inputs/day9.txt");
}
