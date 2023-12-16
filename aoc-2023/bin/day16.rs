use aoc_2023::day16;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 16: The Floor Will Be Lava")
        .solution(|iter| day16::count_shining_tiles(iter))
        .run("inputs/day16.txt");
}
