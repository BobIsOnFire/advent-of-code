use aoc_2024::day23;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 23: DISPLAY NAME")
        .solution(|iter| day23::get_answer(iter))
        .run("inputs/day23.txt");
}
