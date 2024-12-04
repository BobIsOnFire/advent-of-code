use aoc_2024::day4;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 4: Ceres Search")
        .solution(|iter| day4::get_answer(iter))
        .run("inputs/day4.txt");
}
