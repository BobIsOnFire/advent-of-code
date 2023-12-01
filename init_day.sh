#!/bin/bash

set -e

num=$1

if ! test $num; then
    echo "Set day number!"
    exit 1
fi

year=$2; test $year || year=2023

cd aoc-${year}
# Prepare directories
mkdir -p src/day${num}/
mkdir -p bin/
mkdir -p inputs/

# Library code
cat <<EOF >src/day${num}/mod.rs
use aoc_common::util;

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    Ok((0, 0))
}
EOF

touch src/day${num}/mod.rs
echo "pub mod day${num};" >>src/lib.rs

# Input (cannot download without logging in...)
touch inputs/day${num}.txt

# Executable
cat <<EOF >bin/day${num}.rs
use aoc_${year}::day${num};
use aoc_common::Solution;

fn main() {
    Solution::new("Day ${num}: DISPLAY NAME")
        .solution(|iter| day${num}::get_answer(iter))
        .run("inputs/day${num}.txt");
}
EOF

cat <<EOF >>Cargo.toml

[[bin]]
name = "day${num}"
path = "bin/day${num}.rs"
EOF

# Format code, verify that template works

cargo fmt
cargo run --bin=day${num}
