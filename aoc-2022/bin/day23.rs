use aoc_2022::day23;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 23: Unstable Diffusion")
        .solution(|iter| day23::spread_elves(iter))
        .run("inputs/day23.txt");
}
