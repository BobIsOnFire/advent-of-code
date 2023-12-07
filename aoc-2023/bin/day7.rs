use aoc_2023::day7;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 7: Camel Cards")
        .solution(|iter| day7::play_poker(iter))
        .run("inputs/day7.txt");
}
