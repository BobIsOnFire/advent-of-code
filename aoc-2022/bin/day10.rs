use aoc_2022::day10;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 10: Cathode-Ray Tube")
        .solution(|iter| day10::get_signal_strengths(iter))
        .run("inputs/day10.txt");
}
