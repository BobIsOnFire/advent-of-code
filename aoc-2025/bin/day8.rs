use aoc_2025::day8;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 8: Playground")
        .solution(|iter| day8::connect_boxes(iter))
        .run("inputs/day8.txt");
}
