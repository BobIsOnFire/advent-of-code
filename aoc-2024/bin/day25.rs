use aoc_2024::day25;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 25: DISPLAY NAME")
        .solution(|iter| day25::get_answer(iter))
        .run("inputs/day25.txt");
}
