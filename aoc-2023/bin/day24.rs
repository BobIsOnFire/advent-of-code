use aoc_2023::day24;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 24: Never Tell Me The Odds")
        .solution(|iter| day24::magic_collisions(iter))
        .run("inputs/day24.txt");
}
