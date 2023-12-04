use aoc_2023::day3;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 3: Gear Ratios")
        .solution(|iter| day3::get_answer(iter))
        .run("inputs/day3.txt");
}
