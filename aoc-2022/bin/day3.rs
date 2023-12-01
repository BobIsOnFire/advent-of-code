use aoc_2022::day3;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 3: Rucksack Reorganization")
        .solution(|iter| day3::get_misplacings::<3>(iter))
        .run("inputs/day3.txt");
}
