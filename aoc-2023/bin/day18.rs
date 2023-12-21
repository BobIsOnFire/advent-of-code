use aoc_2023::day18;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 18: Lavaduct Lagoon")
        .solution(|iter| day18::dig_lagoon(iter))
        .run("inputs/day18.txt");
}
