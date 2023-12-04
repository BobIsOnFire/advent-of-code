use aoc_2023::day2;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 2: Cube Conundrum")
        .solution(|iter| day2::play_cube_game(iter))
        .run("inputs/day2.txt");
}
