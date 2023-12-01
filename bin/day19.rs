use aoc2022::day19;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 19: DISPLAY NAME")
        .solution(|iter| day19::get_answer(iter))
        .run("inputs/day19.txt");
}
