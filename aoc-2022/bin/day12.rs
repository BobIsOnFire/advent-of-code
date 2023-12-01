use aoc_2022::day12;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 12: Hill Climbing Algorithm")
        .solution(|iter| day12::find_shortest_distance(iter))
        .run("inputs/day12.txt");
}
