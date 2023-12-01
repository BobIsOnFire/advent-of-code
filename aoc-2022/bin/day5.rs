use aoc_2022::day5;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 5: Supply Stacks")
        .solution(|iter| day5::reorder_stacks::<9>(iter))
        .run("inputs/day5.txt");
}
