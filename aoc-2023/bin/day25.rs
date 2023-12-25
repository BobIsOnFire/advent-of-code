use aoc_2023::day25;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 25: Snowverload")
        .solution(|iter| day25::disconnect_nodes(iter))
        .run("inputs/day25.txt");
}
