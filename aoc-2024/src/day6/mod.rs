mod data;

use data::{Direction, Tile};

use aoc_common::util::{self, BitSet, MatrixIndex, VecMatrix};

use std::convert::TryInto;

#[derive(Clone, Copy)]
struct GuardPosition {
    coord: MatrixIndex,
    direction: Direction,
}

struct Guard {
    tilemap: VecMatrix<Tile>,
    position: GuardPosition,
    visited: VecMatrix<bool>,
    obstructions: VecMatrix<bool>,
}

impl Guard {
    fn new(tilemap: VecMatrix<Tile>) -> Result<Self, String> {
        let position = tilemap
            .iter_enumerate()
            .find_map(|(coord, &tile)| match tile {
                Tile::Start(direction) => Some(GuardPosition { coord, direction }),
                _ => None,
            })
            .ok_or("Start cell was not found")?;

        let mut guard = Self {
            position,
            visited: VecMatrix::with_data(vec![false; tilemap.len()], tilemap.width()),
            obstructions: VecMatrix::with_data(vec![false; tilemap.len()], tilemap.width()),
            tilemap,
        };

        guard.visited[guard.position.coord] = true;

        Ok(guard)
    }

    fn next_position(&self, position: GuardPosition) -> Option<GuardPosition> {
        let next_coord = match position.direction {
            Direction::Up => self.tilemap.next_up(position.coord)?,
            Direction::Right => self.tilemap.next_right(position.coord)?,
            Direction::Down => self.tilemap.next_down(position.coord)?,
            Direction::Left => self.tilemap.next_left(position.coord)?,
        };

        let next_position = match self.tilemap[next_coord] {
            Tile::Obstruction => GuardPosition {
                coord: position.coord,
                direction: position.direction.clockwise(),
            },
            _ => GuardPosition {
                coord: next_coord,
                direction: position.direction,
            },
        };

        Some(next_position)
    }

    fn is_cycled(&self, mut position: GuardPosition) -> bool {
        let mut visited = VecMatrix::with_data(
            vec![BitSet::new(); self.tilemap.len()],
            self.tilemap.width(),
        );

        while let Some(next_position) = self.next_position(position) {
            position = next_position;
            if visited[position.coord].contains(position.direction as u64) {
                return true;
            }
            visited[position.coord].insert(position.direction as u64);
        }

        false
    }

    fn record_moves(&mut self) {
        while let Some(next_position) = self.next_position(self.position) {
            // Try obstructing next tile only if it's not visited yet:
            // - If we obstruct tile which we used in the past to get into current position, we'll
            //   no longer be able to use it to get into this position.
            // - If tile forward to us is already obstructed, `next_position()` will calculate that
            //   we need to turn => position.coord == next_position.coord => next_position.coord will
            //   always be visited => we'll only try obstructing if there's a path forward to us.
            if !self.visited[next_position.coord] {
                self.tilemap[next_position.coord] = Tile::Obstruction;
                self.obstructions[next_position.coord] = self.is_cycled(self.position);
                self.tilemap[next_position.coord] = Tile::Path;
            }

            self.position = next_position;
            self.visited[self.position.coord] = true;
        }
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap: VecMatrix<Tile> = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(|byte| byte as char)
            .map(TryInto::try_into)
            .collect::<Result<Vec<_>, _>>()?;

        VecMatrix::with_data(data, width)
    };

    let mut guard = Guard::new(tilemap)?;
    guard.record_moves();

    let visited_count = guard.visited.into_iter().filter(|c| *c).count();
    let obstruction_count = guard.obstructions.into_iter().filter(|c| *c).count();

    Ok((visited_count, obstruction_count))
}
