use aoc2022::day18;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 18: Boiling Boulders")
        .solution(|iter| day18::find_surface_area(iter))
        .run("inputs/day18.txt");
}
