use aoc2022::day2;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 2: Rock Paper Scissors")
        .solution(|iter| day2::get_total_scores(iter))
        .run("inputs/day2.txt");
}
