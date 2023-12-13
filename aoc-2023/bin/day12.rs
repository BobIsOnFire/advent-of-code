use aoc_2023::day12;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 12: Hot Springs")
        .solution(|iter| day12::count_possible_states(iter))
        .run("inputs/day12.txt");
}
