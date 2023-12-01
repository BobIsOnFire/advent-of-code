use aoc2022::day8;
use aoc2022::Solution;

fn main() {
    Solution::new("Day 8: Treetop Tree House")
        .solution(|iter| day8::find_visible_trees(iter))
        .run("inputs/day8.txt");
}
