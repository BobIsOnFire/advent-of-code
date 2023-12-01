use aoc2022::day1;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 1: Calorie Counting")
        .solution(|iter| day1::get_n_highest::<3>(iter))
        .run("inputs/day1.txt");
}
