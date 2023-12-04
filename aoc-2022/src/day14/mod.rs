use aoc_common::util::{self, MatrixIndex, VecMatrix};

type Coord = MatrixIndex;

const SAND_SOURCE: Coord = Coord { row: 0, col: 500 };
const FLOOR_OFFSET: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Stone,
    Air,
    Sand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StoneLine {
    from: Coord,
    to: Coord,
}

impl StoneLine {
    fn try_new(from: Coord, to: Coord) -> util::GenericResult<Self> {
        if from.row != to.row && from.col != to.col {
            return Err(format!(
                "Two line coordinates do not form a vertical or horizontal line: {},{} -> {},{}",
                from.col, from.row, to.col, to.row
            )
            .into());
        }

        Ok(Self { from, to })
    }

    fn iter_tiles(&self) -> impl Iterator<Item = Coord> {
        let &Self {
            from: Coord { row: from_row, col: from_col },
            to: Coord { row: to_row, col: to_col },
        } = self;

        if from_row == to_row {
            // Horizontal line
            let (from_col, to_col) = (usize::min(from_col, to_col), usize::max(from_col, to_col));
            (from_col..=to_col).map(Box::new(move |col| Coord { row: to_row, col }) as Box<dyn Fn(usize) -> Coord>)
        } else {
            // Vertical line
            let (from_row, to_row) = (usize::min(from_row, to_row), usize::max(from_row, to_row));
            (from_row..=to_row).map(Box::new(move |row| Coord { row, col: to_col }) as Box<dyn Fn(usize) -> Coord>)
        }
    }
}

struct Cave {
    tilemap: VecMatrix<Tile>,
}

impl Cave {
    fn new(lines: &[StoneLine]) -> Self {
        let max_row = lines.iter().flat_map(|line| [line.from.row, line.to.row]).max().unwrap();

        let width = 1000;
        let height = max_row + FLOOR_OFFSET;

        let mut tilemap = VecMatrix::with_data(vec![Tile::Air; width * height], width);
        for line in lines {
            for tile_coord in line.iter_tiles() {
                tilemap[tile_coord] = Tile::Stone;
            }
        }

        Self { tilemap }
    }

    fn place_sand_unit(&mut self, sand_source: Coord) -> Option<Coord> {
        if self.tilemap[sand_source] != Tile::Air {
            return None;
        }

        let mut sand_unit = sand_source;
        loop {
            // Have we reached bottom of the cave?
            if sand_unit.row == self.tilemap.height() - 1 {
                // There is floor at the bottom -- place it in
                self.tilemap[sand_unit] = Tile::Sand;
                break Some(sand_unit);
            }

            // Can we fall down below?
            let down_below = Coord {
                row: sand_unit.row + 1,
                col: sand_unit.col,
            };
            if self.tilemap[down_below] == Tile::Air {
                sand_unit = down_below;
                continue;
            }

            // Can we fall down-left?
            let down_left = Coord {
                row: sand_unit.row + 1,
                col: sand_unit.col - 1,
            };
            if self.tilemap[down_left] == Tile::Air {
                sand_unit = down_left;
                continue;
            }

            // Can we fall down-right?
            let down_right = Coord {
                row: sand_unit.row + 1,
                col: sand_unit.col + 1,
            };
            if self.tilemap[down_right] == Tile::Air {
                sand_unit = down_right;
                continue;
            }

            // If we can't fall anywhere, looks like we reached a stable point
            self.tilemap[sand_unit] = Tile::Sand;
            break Some(sand_unit);
        }
    }

    fn get_floor_level(&self) -> usize {
        self.tilemap.height()
    }
}

fn parse_coords(lexer: &mut util::Lexer<'_>) -> util::lexer::Result<Coord> {
    let col = lexer.unsigned_number()?;
    lexer.literal(",")?;
    let row = lexer.unsigned_number()?;
    Ok(Coord { row, col })
}

pub fn count_stable_units(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut stone_lines: Vec<StoneLine> = vec![];

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let mut prev = parse_coords(&mut lexer)?;
        while lexer.end().is_err() {
            lexer.literal(" -> ")?;
            let coord = parse_coords(&mut lexer)?;

            stone_lines.push(StoneLine::try_new(prev, coord)?);
            prev = coord;
        }
    }

    let mut cave = Cave::new(&stone_lines);
    let sand_units: Vec<_> = std::iter::from_fn(|| cave.place_sand_unit(SAND_SOURCE)).collect();

    let floor = cave.get_floor_level();

    // Count all sand units until the first one falls on the floor; this will give
    // us number of "stable" units until some of them start falling into the void (Part 1)
    let sand_count_no_floor = sand_units
        .iter()
        // coord.row + 1 will give us coordinate of tile where current sand unit is placed
        .take_while(|coord| coord.row + 1 < floor)
        .count();
    let sand_count = sand_units.len();

    Ok((sand_count_no_floor, sand_count))
}
