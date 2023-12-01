use aoc_common::util::{self, MatrixIndex};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Filled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockMove {
    Left,
    Right,
    Down,
}

const fn create_bitrow<const W: usize>(row: [Tile; W]) -> u16 {
    debug_assert!(W < u16::BITS as usize);

    // iter() is not a const fn. Sucks
    // row.iter().fold(0, |acc, x| (acc << 1) & *x as u16)

    let mut bitrow = 0;
    let mut i = 0;
    while i < W {
        bitrow = (bitrow << 1) | row[i] as u16;
        i += 1;
    }
    bitrow
}

const fn create_bitmap<const W: usize, const H: usize>(data: [[Tile; W]; H]) -> [u16; H] {
    // map() is not a const fn. Sucks
    // data.map(create_bitrow)

    let mut result = [0; H];
    let mut i = 0;
    while i < H {
        result[i] = create_bitrow(data[i]);
        i += 1;
    }
    result
}

const fn create_line<const LEN: usize>() -> u16 {
    create_bitrow([Tile::Filled; LEN])
}

const fn create_walls<const LEN: usize>() -> u16 {
    let mut row = [Tile::Empty; LEN];
    row[0] = Tile::Filled;
    row[LEN - 1] = Tile::Filled;
    create_bitrow(row)
}

fn draw_bitrow<const LEN: usize>(mut row: u16) {
    row = row.reverse_bits();
    for _ in 0..LEN {
        if (1u16 << (u16::BITS - 1)) & row == 0 {
            print!(".");
        } else {
            print!("#");
        }
        row <<= 1;
    }
    println!();
}

#[derive(Clone)]
struct Piece {
    data: Vec<u16>,
}

impl Piece {
    fn horizontal_line() -> Self {
        Self {
            data: [create_line::<4>()].into(),
        }
    }

    fn vertical_line() -> Self {
        Self {
            data: [create_line::<1>(); 4].into(),
        }
    }

    fn square() -> Self {
        Self {
            data: [create_line::<2>(); 2].into(),
        }
    }

    fn plus_sign() -> Self {
        use Tile::*;
        Self {
            data: create_bitmap([
                [Empty, Filled, Empty],
                [Filled, Filled, Filled],
                [Empty, Filled, Empty],
            ])
            .into(),
        }
    }

    fn angle() -> Self {
        use Tile::*;
        Self {
            data: create_bitmap([
                [Filled, Filled, Filled],
                [Filled, Empty, Empty],
                [Filled, Empty, Empty],
            ])
            .into(),
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

    fn height(&self) -> usize {
        self.data.len()
    }
}

#[derive(Clone)]
struct Chamber {
    tilemap: Vec<u16>,
}

impl Chamber {
    fn new() -> Self {
        Self {
            tilemap: [create_line::<9>()].into(),
        }
    }

    fn add_level(&mut self) {
        self.tilemap.push(create_walls::<9>());
    }

    fn height(&self) -> usize {
        self.tilemap.len()
    }

    fn get_chamber_row(&self, row: usize) -> u16 {
        if row < self.tilemap.len() {
            self.tilemap[row]
        } else {
            create_walls::<9>()
        }
    }

    fn place(&mut self, piece: &Piece, at: MatrixIndex) {
        let MatrixIndex { row, col } = at;

        let levels_to_add = usize::saturating_sub(row + piece.height(), self.height());
        for _ in 0..levels_to_add {
            self.add_level()
        }

        for (row_num, &row_data) in piece.data.iter().enumerate() {
            self.tilemap[row + row_num] |= row_data << col;
        }
    }

    fn check_collision(&self, piece: &Piece, at: MatrixIndex, rock_move: RockMove) -> bool {
        let MatrixIndex { mut row, mut col } = at;

        match rock_move {
            RockMove::Down => row -= 1,
            RockMove::Left => col -= 1,
            RockMove::Right => col += 1,
        };

        for (row_num, &row_data) in piece.data.iter().enumerate() {
            if self.get_chamber_row(row + row_num) & (row_data << col) != 0 {
                return true;
            }
        }

        false
    }

    #[allow(unused)]
    fn draw(&self) {
        for &(mut row_data) in self.tilemap.iter() {
            draw_bitrow::<9>(row_data);
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

    let mut chamber = Chamber::new();

    for shape in Piece::all_shapes().into_iter().cycle().take(2022) {
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
                chamber.place(&shape, rock_position);
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
