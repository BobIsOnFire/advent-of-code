use aoc_2025::day10;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 10: Factory")
        .solution(|iter| day10::count_button_presses(iter))
        .run("inputs/day10.txt");
}
