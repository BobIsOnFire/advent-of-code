use aoc_2022::day8;
use aoc_common::Solution;

fn main() {
    Solution::new("Day 8: Treetop Tree House")
        .solution(|iter| day8::find_visible_trees(iter))
        .run("inputs/day8.txt");
}
