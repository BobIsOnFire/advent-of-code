use std::collections::HashSet;

use crate::util;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Block {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum Orientation {
    X_UP,
    X_DOWN,
    Y_UP,
    Y_DOWN,
    Z_UP,
    Z_DOWN,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Surface {
    block: Block,
    orientation: Orientation,
}

impl Block {
    fn get_common_surface(&self, other: &Self) -> Option<Surface> {
        let orient = match (other.x - self.x, other.y - self.y, other.z - self.z) {
            (-1, 0, 0) => Some(Orientation::X_DOWN),
            (1, 0, 0) => Some(Orientation::X_UP),
            (0, -1, 0) => Some(Orientation::Y_DOWN),
            (0, 1, 0) => Some(Orientation::Y_UP),
            (0, 0, -1) => Some(Orientation::Z_DOWN),
            (0, 0, 1) => Some(Orientation::Z_UP),
            _ => None,
        };

        orient.map(|orientation| Surface {
            block: *self,
            orientation,
        })
    }
}

impl Orientation {
    fn all() -> [Self; 6] {
        [
            Self::X_UP,
            Self::X_DOWN,
            Self::Y_UP,
            Self::Y_DOWN,
            Self::Z_UP,
            Self::Z_DOWN,
        ]
    }

    fn inverse(&self) -> Self {
        match self {
            Self::X_UP => Self::X_DOWN,
            Self::X_DOWN => Self::X_UP,
            Self::Y_UP => Self::Y_DOWN,
            Self::Y_DOWN => Self::Y_UP,
            Self::Z_UP => Self::Z_DOWN,
            Self::Z_DOWN => Self::Z_UP,
        }
    }
}

impl Surface {
    fn block_surfaces(block: Block) -> [Self; 6] {
        Orientation::all().map(|orientation| Self { block, orientation })
    }

    fn get_alternative(&self) -> Self {
        let mut result = Self {
            block: self.block,
            orientation: self.orientation.inverse(),
        };

        use Orientation::*;
        match self.orientation {
            X_UP => result.block.x += 1,
            X_DOWN => result.block.x -= 1,
            Y_UP => result.block.y += 1,
            Y_DOWN => result.block.y -= 1,
            Z_UP => result.block.z += 1,
            Z_DOWN => result.block.z -= 1,
        };

        result
    }

    fn get_side_blocks(&self) -> [Block; 4] {
        let mut result = [self.block; 4];

        use Orientation::*;
        match self.orientation {
            X_UP | X_DOWN => {
                result[0].y -= 1;
                result[1].y += 1;
                result[2].z -= 1;
                result[3].z += 1;
            }
            Y_UP | Y_DOWN => {
                result[0].x -= 1;
                result[1].x += 1;
                result[2].z -= 1;
                result[3].z += 1;
            }
            Z_UP | Z_DOWN => {
                result[0].x -= 1;
                result[1].x += 1;
                result[2].y -= 1;
                result[3].y += 1;
            }
        }

        result
    }
}

pub fn find_surface_area(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut blocks = HashSet::new();
    let mut surfaces = HashSet::new();
    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let x = lexer.number()?;
        lexer.literal(",")?;
        let y = lexer.number()?;
        lexer.literal(",")?;
        let z = lexer.number()?;
        lexer.end()?;

        let block = Block { x, y, z };

        blocks.insert(block);

        for surface in Surface::block_surfaces(block) {
            let alter = surface.get_alternative();
            if !surfaces.remove(&alter) {
                surfaces.insert(surface);
            }
        }
    }

    let total_area = surfaces.len();

    let surface_start = Surface {
        block: *blocks.iter().max_by_key(|b| b.x).unwrap(),
        orientation: Orientation::X_UP,
    };

    assert!(surfaces.contains(&surface_start));
    let mut visited = HashSet::from([surface_start]);
    let mut visit_stack = vec![surface_start];

    while let Some(surface) = visit_stack.pop() {
        let alter = surface.get_alternative();
        let top = alter.block;
        for (side, top_side) in surface
            .get_side_blocks()
            .into_iter()
            .zip(alter.get_side_blocks())
        {
            let connected_surface = match (blocks.contains(&top_side), blocks.contains(&side)) {
                (true, _) => top_side.get_common_surface(&top),
                (false, true) => side.get_common_surface(&top_side),
                (false, false) => surface.block.get_common_surface(&side),
            }
            .unwrap();

            if !visited.contains(&connected_surface) {
                assert!(surfaces.contains(&connected_surface));
                visited.insert(connected_surface);
                visit_stack.push(connected_surface);
            }
        }
    }

    let outside_area = visited.len();

    Ok((total_area, outside_area))
}
