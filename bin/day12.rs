use aoc2022::day12;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 12: Hill Climbing Algorithm")
        .solution(|iter| day12::find_shortest_distance(iter))
        .run("inputs/day12.txt");
}
