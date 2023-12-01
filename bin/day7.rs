use aoc2022::day7;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 7: No Space Left On Device")
        .solution(|iter| day7::get_directory_sizes(iter))
        .run("inputs/day7.txt");
}
