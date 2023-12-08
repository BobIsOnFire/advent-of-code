use aoc_2023::day8;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 8: Haunted Wasteland")
        .solution(|iter| day8::count_steps(iter))
        .run("inputs/day8.txt");
}
