use aoc_2023::day21;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 21: Step Counter")
        .solution(|iter| day21::count_garden_steps(iter))
        .run("inputs/day21.txt");
}
