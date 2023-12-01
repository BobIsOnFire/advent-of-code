use aoc2022::day5;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 5: Supply Stacks")
        .solution(|iter| day5::reorder_stacks::<9>(iter))
        .run("inputs/day5.txt");
}
