use aoc2022::day3;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 3: Rucksack Reorganization")
        .solution(|iter| day3::get_misplacings::<3>(iter))
        .run("inputs/day3.txt");
}
