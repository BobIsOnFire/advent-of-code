use aoc_2024::day5;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 5: Print Queue")
        .solution(|iter| day5::order_updates(iter))
        .run("inputs/day5.txt");
}
