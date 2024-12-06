use aoc_2024::day6;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 6: DISPLAY NAME")
        .solution(|iter| day6::get_answer(iter))
        .run("inputs/day6.txt");
}
