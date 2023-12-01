use aoc2022::day6;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 6: Tuning Trouble")
        .solution(|iter| day6::find_markers::<4, 14>(iter))
        .run("inputs/day6.txt");
}
