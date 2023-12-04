use aoc_2023::day4;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 4: Scratchcards")
        .solution(|iter| day4::count_scratchcards(iter))
        .run("inputs/day4.txt");
}
