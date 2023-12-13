use aoc_2023::day13;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 13: Point of Incidence")
        .solution(|iter| day13::count_mirrors(iter))
        .run("inputs/day13.txt");
}
