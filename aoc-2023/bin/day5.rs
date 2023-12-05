use aoc_2023::day5;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 5: If You Give A Seed A Fertilizer")
        .solution(|iter| day5::find_locations(iter))
        .run("inputs/day5.txt");
}
