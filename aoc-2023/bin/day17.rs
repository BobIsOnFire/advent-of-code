use aoc_2023::day17;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 17: Clumsy Crucible")
        .solution(|iter| day17::find_shortest_paths(iter))
        .run("inputs/day17.txt");
}
