use aoc_2024::day17;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 17: DISPLAY NAME")
        .solution(|iter| day17::get_answer(iter))
        .run("inputs/day17.txt");
}
