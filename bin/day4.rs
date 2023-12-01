use aoc2022::day4;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 4: Camp Cleanup")
        .solution(|iter| day4::count_overlaps(iter))
        .run("inputs/day4.txt");
}
