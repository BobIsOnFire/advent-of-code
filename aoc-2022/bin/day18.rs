use aoc_2022::day18;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 18: Boiling Boulders")
        .solution(|iter| day18::find_surface_area(iter))
        .run("inputs/day18.txt");
}
