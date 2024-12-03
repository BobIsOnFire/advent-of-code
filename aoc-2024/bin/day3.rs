use aoc_2024::day3;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 3: Mull It Over")
        .solution(|iter| day3::parse_corrupted_data(iter))
        .run("inputs/day3.txt");
}
