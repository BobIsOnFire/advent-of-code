use crate::util::{self, MatrixIndex, VecMatrix};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Filled,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockMove {
    Left,
    Right,
    Down,
}

#[derive(Clone)]
struct Rock {
    tilemap: VecMatrix<Tile>,
}

impl Rock {
    fn horizontal_line() -> Self {
        use Tile::*;
        #[rustfmt::skip]
        let shape = vec![
            Filled, Filled, Filled, Filled,
        ];
        Self {
            tilemap: VecMatrix::with_data(shape, 4),
        }
    }

    fn vertical_line() -> Self {
        use Tile::*;
        #[rustfmt::skip]
        let shape = vec![
            Filled,
            Filled,
            Filled,
            Filled,
        ];
        Self {
            tilemap: VecMatrix::with_data(shape, 1),
        }
    }

    fn square() -> Self {
        use Tile::*;
        #[rustfmt::skip]
        let shape = vec![
            Filled, Filled,
            Filled, Filled,
        ];
        Self {
            tilemap: VecMatrix::with_data(shape, 2),
        }
    }

    fn plus_sign() -> Self {
        use Tile::*;
        #[rustfmt::skip]
        let shape = vec![
            Empty,  Filled, Empty,
            Filled, Filled, Filled,
            Empty,  Filled, Empty,
        ];
        Self {
            tilemap: VecMatrix::with_data(shape, 3),
        }
    }

    fn angle() -> Self {
        use Tile::*;
        #[rustfmt::skip]
        let shape = vec![
            Filled, Filled, Filled,
            Empty,  Empty,  Filled,
            Empty,  Empty,  Filled,
        ];
        Self {
            tilemap: VecMatrix::with_data(shape, 3),
        }
    }

    fn all_shapes() -> [Self; 5] {
        [
            Self::horizontal_line(),
            Self::plus_sign(),
            Self::angle(),
            Self::vertical_line(),
            Self::square(),
        ]
    }

    fn new_chamber() -> Self {
        Self {
            tilemap: VecMatrix::with_data(vec![Tile::Filled; 9], 9),
        }
    }

    fn add_level(&mut self) {
        self.tilemap.push(Tile::Filled);
        self.tilemap.extend([Tile::Empty; 7]);
        self.tilemap.push(Tile::Filled);
    }

    fn width(&self) -> usize {
        self.tilemap.width()
    }

    fn height(&self) -> usize {
        self.tilemap.height()
    }

    fn get_chamber_tile(&self, at: MatrixIndex) -> Tile {
        if at.row < self.tilemap.height() {
            self.tilemap[at]
        } else if (1..=7).contains(&at.col) {
            Tile::Empty
        } else {
            Tile::Filled
        }
    }

    fn copy_tiles(&mut self, other: &Self, at: MatrixIndex) {
        let MatrixIndex { row, col } = at;

        let levels_to_add = usize::saturating_sub(row + other.height(), self.height());
        for _ in 0..levels_to_add {
            self.add_level()
        }

        for (idx, tile) in other.tilemap.iter_enumerate() {
            if *tile != Tile::Empty {
                let copy_idx = MatrixIndex {
                    row: row + idx.row,
                    col: col + idx.col,
                };
                if self.tilemap[copy_idx] != Tile::Empty {
                    println!("Oh no! A rock wants to be placed where things collide");
                    println!("Chamber:");
                    self.draw();
                    println!("Rock:");
                    other.draw();
                    println!("Location: {:?}", at);
                }
                self.tilemap[copy_idx] = *tile;
            }
        }
    }

    fn check_collision(&self, other: &Self, at: MatrixIndex, rock_move: RockMove) -> bool {
        let MatrixIndex { row, col } = at;

        let collision_idx = match rock_move {
            RockMove::Down => MatrixIndex { row: row - 1, col },
            RockMove::Left => MatrixIndex { row, col: col - 1 },
            RockMove::Right => MatrixIndex { row, col: col + 1 },
        };

        for (idx, &tile) in other.tilemap.iter_enumerate() {
            let chamber_idx = MatrixIndex {
                row: collision_idx.row + idx.row,
                col: collision_idx.col + idx.col,
            };

            if self.get_chamber_tile(chamber_idx) == Tile::Filled && tile == Tile::Filled {
                return true;
            }
        }

        false
    }

    #[allow(unused)]
    fn draw(&self) {
        for (idx, tile) in self.tilemap.iter_enumerate() {
            print!(
                "{}",
                match tile {
                    Tile::Empty => '.',
                    Tile::Filled => '#',
                }
            );
            if idx.col == self.tilemap.width() - 1 {
                println!();
            }
        }
        println!();
    }
}

pub fn tetris_simulator(
    mut lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let line = lines
        .next()
        .expect("A line with move data should be provided");

    let mut side_moves = line
        .chars()
        .map(|ch| match ch {
            '<' => RockMove::Left,
            '>' => RockMove::Right,
            _ => panic!("Unknown move char: {}", ch),
        })
        .cycle();

    let mut chamber = Rock::new_chamber();

    for shape in Rock::all_shapes().into_iter().cycle().take(2022) {
        let mut rock_position = MatrixIndex {
            row: chamber.height() + 3,
            col: 3,
        };
        loop {
            let side_move = side_moves.next().unwrap();
            if !chamber.check_collision(&shape, rock_position, side_move) {
                match side_move {
                    RockMove::Left => rock_position.col -= 1,
                    RockMove::Right => rock_position.col += 1,
                    _ => (),
                };
            }

            if chamber.check_collision(&shape, rock_position, RockMove::Down) {
                chamber.copy_tiles(&shape, rock_position);
                break;
            } else {
                rock_position.row -= 1;
            }
        }
        // chamber.draw();
    }

    // -1 to remove chamber floor from count
    let chamber_height = chamber.height() - 1;

    Ok((chamber_height, 0))
}
