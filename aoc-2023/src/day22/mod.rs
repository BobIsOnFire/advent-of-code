use std::collections::{HashMap, HashSet};

use aoc_common::util;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, PartialEq, Eq)]
struct Brick {
    idx: usize,
    from: Coord,
    to: Coord,
}

fn add_brick(brick: &Brick, coord_to_idx: &mut HashMap<Coord, usize>) {
    for x in brick.from.x..=brick.to.x {
        for y in brick.from.y..=brick.to.y {
            for z in brick.from.z..=brick.to.z {
                coord_to_idx.insert(Coord { x, y, z }, brick.idx);
            }
        }
    }
}

fn remove_brick(brick: &Brick, coord_to_idx: &mut HashMap<Coord, usize>) {
    for x in brick.from.x..=brick.to.x {
        for y in brick.from.y..=brick.to.y {
            for z in brick.from.z..=brick.to.z {
                coord_to_idx.remove(&Coord { x, y, z });
            }
        }
    }
}

fn check_stable(brick: &Brick, coord_to_idx: &HashMap<Coord, usize>) -> bool {
    if brick.from.z == 1 {
        return true;
    }

    for x in brick.from.x..=brick.to.x {
        for y in brick.from.y..=brick.to.y {
            if coord_to_idx.contains_key(&Coord { z: brick.from.z - 1, x, y }) {
                return true;
            }
        }
    }

    false
}

fn do_count_falling_bricks(
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
    current: usize,
    falling: &mut HashSet<usize>,
) {
    if supported_by[current].is_subset(falling) {
        falling.insert(current);
        for next in &supports[current] {
            do_count_falling_bricks(supports, supported_by, *next, falling);
        }
    }
}

fn count_falling_bricks(
    supports: &[HashSet<usize>],
    supported_by: &[HashSet<usize>],
    start: usize,
) -> usize {
    let mut falling = HashSet::from([start]);
    for next in &supports[start] {
        do_count_falling_bricks(supports, supported_by, *next, &mut falling);
    }
    falling.len() - 1
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut bricks = lines
        .enumerate()
        .map(|(idx, line)| {
            let mut lexer = util::Lexer::of(&line);
            let mut brick = Brick {
                idx,
                from: Coord::default(),
                to: Coord::default(),
            };

            lexer
                .chain()
                .number(&mut brick.from.x)?
                .literal(",")?
                .number(&mut brick.from.y)?
                .literal(",")?
                .number(&mut brick.from.z)?
                .literal("~")?
                .number(&mut brick.to.x)?
                .literal(",")?
                .number(&mut brick.to.y)?
                .literal(",")?
                .number(&mut brick.to.z)?
                .end()?;

            if brick.from.z > brick.to.z {
                std::mem::swap(&mut brick.from, &mut brick.to);
            }

            Ok(brick)
        })
        .collect::<util::lexer::Result<Vec<_>>>()?;

    let mut coord_to_idx = HashMap::new();
    for brick in &bricks {
        add_brick(brick, &mut coord_to_idx);
    }

    // Lower bricks should be falling first
    bricks.sort_unstable_by_key(|brick| brick.from.z);

    for brick in &mut bricks {
        remove_brick(brick, &mut coord_to_idx);
        while !check_stable(brick, &coord_to_idx) {
            brick.from.z -= 1;
            brick.to.z -= 1;
        }
        add_brick(brick, &mut coord_to_idx);
    }

    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut supported_by = vec![HashSet::new(); bricks.len()];

    for brick in &bricks {
        for x in brick.from.x..=brick.to.x {
            for y in brick.from.y..=brick.to.y {
                if let Some(idx) = coord_to_idx.get(&Coord { z: brick.to.z + 1, x, y }) {
                    supports[brick.idx].insert(*idx);
                }
                if let Some(idx) = coord_to_idx.get(&Coord { z: brick.from.z - 1, x, y }) {
                    supported_by[brick.idx].insert(*idx);
                }
            }
        }
    }

    let mut safe_to_remove = 0;
    let mut falling_total = 0;

    for i in 0..bricks.len() {
        let falling = count_falling_bricks(&supports, &supported_by, i);
        falling_total += falling;
        if falling == 0 {
            safe_to_remove += 1;
        }
    }

    Ok((safe_to_remove, falling_total))
}
