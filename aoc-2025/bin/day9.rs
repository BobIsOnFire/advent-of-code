use aoc_2025::day9;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 9: Movie Theater")
        .solution(|iter| day9::find_largest_rectangle(iter))
        .run("inputs/day9.txt");
}
