use aoc_2024::day21;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 21: DISPLAY NAME")
        .solution(|iter| day21::get_answer(iter))
        .run("inputs/day21.txt");
}
