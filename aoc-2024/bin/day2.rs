use aoc_2024::day2;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 2: Red-Nosed Reports")
        .solution(|iter| day2::count_safe_systems(iter))
        .run("inputs/day2.txt");
}
