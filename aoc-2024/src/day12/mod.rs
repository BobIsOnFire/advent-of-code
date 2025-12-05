use aoc_common::util::{self, MatrixIndex, VecMatrix};

#[derive(Clone, Debug)]
struct Region {
    area: usize,
    perimeter: usize,
    sides: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

const fn next_tile(
    tilemap: &VecMatrix<char>,
    tile: MatrixIndex,
    side: Side,
) -> Option<MatrixIndex> {
    match side {
        Side::Top => tilemap.next_up(tile),
        Side::Bottom => tilemap.next_down(tile),
        Side::Left => tilemap.next_left(tile),
        Side::Right => tilemap.next_right(tile),
    }
}

fn is_border_piece(tilemap: &VecMatrix<char>, tile: MatrixIndex, side: Side) -> bool {
    // Check if there's a region border piece on `side` of the `tile`:
    // - If there's a tile of the same region on `side` of the `tile`, border does not exist;
    // - Otherwise (if there's a tile of a different region or no tile at all), there is a border.
    next_tile(tilemap, tile, side).is_none_or(|opposite| tilemap[tile] != tilemap[opposite])
}

fn extends_region_border(tilemap: &VecMatrix<char>, tile: MatrixIndex, side: Side) -> bool {
    // Check if border piece in question is an extension of existing border. Two conditions should apply:
    // - Previous tile is a part of the same region as our tile;
    // - There's a border to extend, i.e. previous tile has border piece on the same side as our tile.
    //   .   .   .
    //  ___(___)___
    //   A  (A)  A

    // We determine direction as top-down, left-right, meaning that "previous" tile for horizontal
    // borders (Top and Bottom sides of the tile) is on the Left, and for vertical borders (Left and
    // Right) -- on the Top. It doesn't actually matter which direction we use, as long as it's
    // consistent in all checks.
    let prev_side = match side {
        Side::Bottom | Side::Top => Side::Left,
        Side::Left | Side::Right => Side::Top,
    };

    #[allow(clippy::option_if_let_else)] // Don't agree, it's MUCH less readable with .map_or()
    if let Some(prev) = next_tile(tilemap, tile, prev_side) {
        let prev_same_region = tilemap[tile] == tilemap[prev];
        let prev_has_border = is_border_piece(tilemap, prev, side);
        prev_same_region && prev_has_border
    } else {
        // Obviously, there's no border to extend if there's no previous tile available.
        false
    }
}

fn get_region(
    tilemap: &VecMatrix<char>,
    start_tile: MatrixIndex,
    id: usize,
    region_ids: &mut VecMatrix<usize>,
) -> Region {
    assert!(region_ids[start_tile] == 0);
    region_ids[start_tile] = id;

    let mut tile_stack = vec![start_tile];
    let mut region = Region { area: 0, perimeter: 0, sides: 0 };

    while let Some(tile) = tile_stack.pop() {
        region.area += 1;
        for side in [Side::Top, Side::Bottom, Side::Left, Side::Right] {
            if is_border_piece(tilemap, tile, side) {
                region.perimeter += 1;

                // If a tile has a border which doesn't extend existing region side, register it as
                // a start of a new side.
                if !extends_region_border(tilemap, tile, side) {
                    region.sides += 1;
                }
            }

            if let Some(neighbor) = next_tile(tilemap, tile, side)
                && tilemap[tile] == tilemap[neighbor]
                && region_ids[neighbor] == 0
            {
                region_ids[neighbor] = id;
                tile_stack.push(neighbor);
            }
        }
    }

    region
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let tilemap: VecMatrix<char> = {
        let mut lines = lines.peekable();

        let width = lines.by_ref().peek().map_or(0, String::len);
        let data = lines
            .flat_map(String::into_bytes)
            .map(Into::into)
            .collect::<Vec<_>>();

        VecMatrix::with_data(data, width)
    };

    let mut region_ids = VecMatrix::with_data(vec![0; tilemap.len()], tilemap.width());
    let mut regions_count = 1;

    let mut price_by_perimeter = 0;
    let mut price_by_sides = 0;

    for (tile, _) in tilemap.iter_enumerate() {
        if region_ids[tile] > 0 {
            continue;
        }
        let next_id = regions_count;
        let region = get_region(&tilemap, tile, next_id, &mut region_ids);

        regions_count += 1;
        price_by_perimeter += region.area * region.perimeter;
        price_by_sides += region.area * region.sides;
    }

    Ok((price_by_perimeter, price_by_sides))
}
