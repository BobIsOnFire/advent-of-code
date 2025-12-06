use aoc_2025::day6;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 6: Trash Compactor")
        .solution(|iter| day6::do_maths(iter))
        .run("inputs/day6.txt");
}
