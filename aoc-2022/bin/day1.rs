use aoc_2022::day1;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 1: Calorie Counting")
        .solution(|iter| day1::get_n_highest::<3>(iter))
        .run("inputs/day1.txt");
}
