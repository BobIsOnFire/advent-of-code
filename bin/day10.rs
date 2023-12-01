use aoc2022::day10;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 10: Cathode-Ray Tube")
        .solution(|iter| day10::get_signal_strengths(iter))
        .run("inputs/day10.txt");
}
