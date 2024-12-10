use std::collections::{HashMap, HashSet};

use aoc_common::util::{self, MatrixIndex, VecMatrix};

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap: VecMatrix<u8> = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|b| b - b'0')
            .collect::<Vec<_>>();

        VecMatrix::with_data(data, width)
    };

    let mut slopes = HashMap::new();
    for (idx, tile) in tilemap.iter_enumerate() {
        slopes.entry(*tile).or_insert(vec![]).push(idx);
    }

    let mut reachables: VecMatrix<HashSet<MatrixIndex>> =
        VecMatrix::with_data(vec![HashSet::new(); tilemap.len()], tilemap.width());
    let mut ratings: VecMatrix<usize> =
        VecMatrix::with_data(vec![0usize; tilemap.len()], tilemap.width());

    for &idx in slopes.get(&9).unwrap_or(&vec![]) {
        reachables[idx].insert(idx);
        ratings[idx] = 1;
    }

    for slope in (0..=8).rev() {
        for &idx in slopes.get(&slope).unwrap_or(&vec![]) {
            let adjacents = std::iter::empty()
                .chain(tilemap.next_left(idx))
                .chain(tilemap.next_down(idx))
                .chain(tilemap.next_right(idx))
                .chain(tilemap.next_up(idx));

            let mut reach: HashSet<MatrixIndex> = HashSet::new();

            for adj_idx in adjacents {
                if tilemap[adj_idx] == slope + 1 {
                    reach.extend(&reachables[adj_idx]);
                    ratings[idx] += ratings[adj_idx];
                }
            }
            reachables[idx].extend(reach);
        }
    }

    let zero_score = slopes
        .get(&0)
        .unwrap_or(&vec![])
        .iter()
        .map(|&idx| reachables[idx].len())
        .sum();

    let zero_rating = slopes
        .get(&0)
        .unwrap_or(&vec![])
        .iter()
        .map(|&idx| ratings[idx])
        .sum();

    Ok((zero_score, zero_rating))
}
