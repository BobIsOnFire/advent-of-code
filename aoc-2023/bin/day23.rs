use aoc_2023::day23;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 23: A Long Walk")
        .solution(|iter| day23::find_longest_path(iter))
        .run("inputs/day23.txt");
}
