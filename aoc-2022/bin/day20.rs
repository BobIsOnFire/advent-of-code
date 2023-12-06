use aoc_2022::day20;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 20: Grove Positioning System")
        .solution(|iter| day20::decrypt_table(iter))
        .run("inputs/day20.txt");
}
