use aoc_common::util::{self, NumberRange};

const ROW_TO_CHECK: i64 = 2_000_000;

fn sort_and_merge(ranges: &mut Vec<NumberRange>) {
    ranges.sort_unstable_by(NumberRange::started_before);

    let mut merged_ranges: Vec<NumberRange> = Vec::with_capacity(ranges.len());
    merged_ranges.push(NumberRange::Empty);

    for &range in ranges.iter() {
        if range.is_empty() {
            continue;
        }
        let last = merged_ranges.last_mut().unwrap();
        if *last & range == NumberRange::Empty {
            // Two closest ranges don't intersect
            if (*last | range).len() == last.len() + range.len() + 1 {
                // They can still be one after another (e.g. [1,2] and [3,4]), we can merge them
                *last = *last | range;
            } else {
                // No common ground (e.g. [1,2] and [5,6]), create new range
                merged_ranges.push(range);
            }
        } else {
            // Merge new range into previous one
            *last = *last | range;
        }
    }

    *ranges = merged_ranges;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: i64,
    col: i64,
}

struct Sensor {
    sensor_coords: Coord,
    beacon_coords: Coord,
}

impl Sensor {
    const fn beacon_distance(&self) -> u64 {
        i64::abs_diff(self.sensor_coords.col, self.beacon_coords.col) + i64::abs_diff(self.sensor_coords.row, self.beacon_coords.row)
    }

    const fn row_distance(&self, row: i64) -> u64 {
        i64::abs_diff(self.sensor_coords.row, row)
    }

    const fn get_covered_rows(&self) -> NumberRange {
        NumberRange::new(
            self.sensor_coords.row - self.beacon_distance() as i64,
            self.sensor_coords.row + self.beacon_distance() as i64,
        )
    }

    const fn get_covered_cells(&self, row: i64) -> NumberRange {
        let delta = self.beacon_distance() as i64 - self.row_distance(row) as i64;
        NumberRange::new(self.sensor_coords.col - delta, self.sensor_coords.col + delta)
    }
}

fn parse_sensor_data(line: &str) -> util::lexer::Result<Sensor> {
    let mut lexer = util::Lexer::of(line);

    lexer.literal("Sensor at x=")?;
    let sensor_col = lexer.number()?;
    lexer.literal(", y=")?;
    let sensor_row = lexer.number()?;
    lexer.literal(": closest beacon is at x=")?;
    let beacon_col = lexer.number()?;
    lexer.literal(", y=")?;
    let beacon_row = lexer.number()?;
    lexer.end()?;

    Ok(Sensor {
        sensor_coords: Coord { col: sensor_col, row: sensor_row },
        beacon_coords: Coord { col: beacon_col, row: beacon_row },
    })
}

#[allow(unused)]
fn draw_sensor_map<const N: usize>(sensors: &[Sensor]) {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        Sensor,
        Beacon,
        Known,
        Unknown,
    }

    let mut tilemap = [[Tile::Unknown; N]; N];
    for sensor in sensors {
        let Coord { row, col } = dbg!(sensor.sensor_coords);
        if (0..20).contains(&row) && (0..20).contains(&col) {
            tilemap[row as usize][col as usize] = Tile::Sensor;
        }

        let Coord { row, col } = sensor.beacon_coords;
        if (0..20).contains(&row) && (0..20).contains(&col) {
            tilemap[row as usize][col as usize] = Tile::Beacon;
        }

        let row_range = sensor.get_covered_rows() & NumberRange::new(0, N as i64 - 1);
        for row in row_range {
            let col_range = sensor.get_covered_cells(row) & NumberRange::new(0, N as i64 - 1);
            for col in col_range {
                if tilemap[row as usize][col as usize] == Tile::Unknown {
                    tilemap[row as usize][col as usize] = Tile::Known;
                }
            }
        }
    }

    for tile_row in tilemap {
        for tile_cell in tile_row {
            print!(
                "{}",
                match tile_cell {
                    Tile::Sensor => 'S',
                    Tile::Beacon => 'B',
                    Tile::Known => '#',
                    Tile::Unknown => '.',
                }
            );
        }
        println!();
    }
}

pub fn find_missing_beacon(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, i64)> {
    let sensors = lines.map(|s| parse_sensor_data(&s)).collect::<Result<Vec<_>, _>>()?;

    // draw_sensor_map::<20>(&sensors);

    let mut ranges: Vec<_> = sensors.iter().map(|sensor| sensor.get_covered_cells(ROW_TO_CHECK)).collect();

    sort_and_merge(&mut ranges);

    let sum = ranges.iter().map(NumberRange::len).sum();

    let mut tuning_freq = 0;
    for row in 0..=4_000_000 {
        let mut ranges: Vec<_> = sensors
            .iter()
            .map(|sensor| sensor.get_covered_cells(row))
            .filter(|range| !range.is_empty())
            .map(|range| range & NumberRange::new(0, 4_000_000))
            .filter(|range| !range.is_empty())
            .collect();
        sort_and_merge(&mut ranges);

        if ranges.len() > 1 {
            let x = ranges[0].len() as i64 + 1;
            let y = row;
            tuning_freq = x * 4_000_000 + y;
        }
    }

    Ok((sum, tuning_freq))
}
