use aoc_2022::day22;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 22: Monkey Map")
        .solution(|iter| day22::traverse_map(iter))
        .run("inputs/day22.txt");
}
