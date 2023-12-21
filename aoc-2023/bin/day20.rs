use aoc_2023::day20;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 20: Pulse Propagation")
        .solution(|iter| day20::press_buttons(iter))
        .run("inputs/day20.txt");
}
