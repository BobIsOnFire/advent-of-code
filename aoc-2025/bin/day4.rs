use aoc_2025::day4;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 4: Printing Department")
        .solution(|iter| day4::remove_paper(iter))
        .run("inputs/day4.txt");
}
