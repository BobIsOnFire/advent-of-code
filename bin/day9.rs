use aoc2022::day9;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 9: Rope Bridge")
        .solution(|iter| day9::count_unique_positions(iter))
        .run("inputs/day9.txt");
}
