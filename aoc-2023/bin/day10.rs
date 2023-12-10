use aoc_2023::day10;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 10: Pipe Maze")
        .solution(|iter| day10::find_enclosing_loop(iter))
        .run("inputs/day10.txt");
}
