use aoc2022::day17;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 17: Pyroclastic Flow")
        .solution(|iter| day17::tetris_simulator(iter))
        .run("inputs/day17.txt");
}
