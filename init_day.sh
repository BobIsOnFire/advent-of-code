#!/bin/bash

set -e

num=$1

if ! test $num; then
    echo "Set day number!"
    exit 1
fi

# Library code
mkdir -p src/day${num}

cat <<EOF >src/day${num}/mod.rs
use crate::util;

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
use aoc2022::day${num};
use aoc2022::Solution;

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
