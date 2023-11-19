use aoc2022::*;

fn main() {
    Solution::new("Day 1: Calorie Counting")
        .solution(|iter| day1::get_n_highest::<3>(iter))
        .run("inputs/day1.txt");

    Solution::new("Day 2: Rock Paper Scissors")
        .solution(|iter| day2::get_total_scores(iter))
        .run("inputs/day2.txt");

    Solution::new("Day 3: Rucksack Reorganization")
        .solution(|iter| day3::get_misplacings::<3>(iter))
        .run("inputs/day3.txt");

    Solution::new("Day 4: Camp Cleanup")
        .solution(|iter| day4::count_overlaps(iter))
        .run("inputs/day4.txt");

    Solution::new("Day 5: Supply Stacks")
        .solution(|iter| day5::reorder_stacks::<9>(iter))
        .run("inputs/day5.txt");

    Solution::new("Day 6: Tuning Trouble")
        .solution(|iter| day6::find_markers::<4, 14>(iter))
        .run("inputs/day6.txt");

    Solution::new("Day 7: No Space Left On Device")
        .solution(|iter| day7::get_directory_sizes(iter))
        .run("inputs/day7.txt");

    Solution::new("Day 8: Treetop Tree House")
        .solution(|iter| day8::find_visible_trees(iter))
        .run("inputs/day8.txt");

    Solution::new("Day 9: Rope Bridge")
        .solution(|iter| day9::count_unique_positions(iter))
        .run("inputs/day9.txt");

    Solution::new("Day 10: Cathode-Ray Tube")
        .solution(|iter| day10::get_signal_strengths(iter))
        .run("inputs/day10.txt");

    Solution::new("Day 12: Hill Climbing Algorithm")
        .solution(|iter| day12::find_shortest_distance(iter))
        .run("inputs/day12.txt");

    Solution::new("Day 13: Distress Signal")
        .solution(|iter| day13::determine_order(iter))
        .run("inputs/day13.txt");

    Solution::new("Day 14: Regolith Reservoir")
        .solution(|iter| day14::count_stable_units(iter))
        .run("inputs/day14.txt");

    // Solution::new("Day 15: Beacon Exclusion Zone")
    //     .solution(|iter| day15::find_missing_beacon(iter))
    //     .run("inputs/day15.txt");

    Solution::new("Day 17: Pyroclastic Flow")
        .solution(|iter| day17::tetris_simulator(iter))
        .run("inputs/day17.txt");
}
