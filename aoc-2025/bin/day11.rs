use aoc_2025::day11;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 11: Reactor")
        .solution(|iter| day11::find_faulty_paths(iter))
        .run("inputs/day11.txt");
}
