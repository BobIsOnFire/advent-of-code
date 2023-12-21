use aoc_2023::day19;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 19: Aplenty")
        .solution(|iter| day19::find_ratings(iter))
        .run("inputs/day19.txt");
}
