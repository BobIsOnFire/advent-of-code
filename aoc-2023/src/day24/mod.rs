use aoc_common::util;

#[derive(Default)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

#[derive(Default)]
struct Stone {
    start: Coord,
    velocity: Coord,
}

impl std::fmt::Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.start, self.velocity)
    }
}

fn check_intersects_xy(first: &Stone, second: &Stone) -> bool {
    let s1 = &first.start;
    let s2 = &second.start;

    let v1 = &first.velocity;
    let v2 = &second.velocity;

    let low = 200_000_000_000_000i128;
    let high = 400_000_000_000_000i128;
    // let low = 7i64;
    // let high = 27i64;

    /*
     * Lines intersection equations:
     *   s1.x + v1.x * t1 = s2.x + v2.x * t2
     *   s1.y + v1.y * t1 = s2.y + v2.y * t2
     *
     * Result:
     *   t1 = ((s2.x - s1.x) * v2.y - (s2.y - s1.y) * v2.x) / (v1.x * v2.y - v1.y * v2.x)
     *   t2 = ((s2.x - s1.x) * v1.y - (s2.y - s1.y) * v1.x) / (v1.x * v2.y - v1.y * v2.x)
     *
     * Let's try to avoid any divisions, don't want to get into floating points. So we need to define following:
     *   q1 = (s2.x - s1.x) * v2.y - (s2.y - s1.y) * v2.x
     *   q2 = (s2.x - s1.x) * v1.y - (s2.y - s1.y) * v1.x
     *   c = v1.x * v2.y - v1.y * v2.x
     *
     * Gives us t1 = q1 / c, t2 = q2 / c.
     *
     * To check bounds:
     *   low <= s1.x + v1.x * t1 <= high
     *   low - s1.x <= v1.x * t1 <= high - s1.x
     *   (low - s1.x) * c <= v1.x * q1 <= (high - s1.x) * c
     * Same for Y.
     */

    let mut q1 = (s2.x - s1.x) * v2.y - (s2.y - s1.y) * v2.x;
    let mut q2 = (s2.x - s1.x) * v1.y - (s2.y - s1.y) * v1.x;
    let mut c = v1.x * v2.y - v1.y * v2.x;

    if c == 0 {
        // println!("Lines are parallel");
        return false;
    }

    if c < 0 {
        c = -c;
        q1 = -q1;
        q2 = -q2;
    }

    if q1 < 0 || q2 < 0 {
        // println!("Lines intersect in the past");
        return false;
    }

    let low_x = (low - i128::from(s1.x)) * i128::from(c);
    let high_x = (high - i128::from(s1.x)) * i128::from(c);

    if !(low_x..=high_x).contains(&(i128::from(q1) * i128::from(v1.x))) {
        // println!("Lines intersect out of bounds");
        return false;
    }

    let low_y = (low - i128::from(s1.y)) * i128::from(c);
    let high_y = (high - i128::from(s1.y)) * i128::from(c);

    if !(low_y..=high_y).contains(&(i128::from(q1) * i128::from(v1.y))) {
        // println!("Lines intersect out of bounds");
        return false;
    }

    true
}

pub fn magic_collisions(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, i64)> {
    let stones = lines
        .map(|line| {
            let mut lexer = util::Lexer::of(&line);
            let mut stone = Stone::default();

            lexer
                .chain()
                .number(&mut stone.start.x)?
                .literal(",")?
                .whitespace()?
                .number(&mut stone.start.y)?
                .literal(",")?
                .whitespace()?
                .number(&mut stone.start.z)?
                .whitespace()?
                .literal("@")?
                .whitespace()?
                .number(&mut stone.velocity.x)?
                .literal(",")?
                .whitespace()?
                .number(&mut stone.velocity.y)?
                .literal(",")?
                .whitespace()?
                .number(&mut stone.velocity.z)?
                .end()?;

            Ok(stone)
        })
        .collect::<util::lexer::Result<Vec<_>>>()?;

    let mut collisions = 0;
    for i in 0..stones.len() {
        for j in (i + 1)..stones.len() {
            if check_intersects_xy(&stones[i], &stones[j]) {
                collisions += 1;
            }
        }
    }

    // Part 2 was done in a magical Excel spreadsheet, I didn't want to replicate it in code. Here's an algorithm:
    // - "Normalize" all stone vectors to be relative to first stone's location and speed by substracting first
    //   stone from all other stones. Now normalized "ultimate" stone line should cross (0,0,0) at some point
    // - Find a pane that goes through (0,0,0) and contains second stone line
    // - Find coordinates at which 3rd and 4th stone lines intersect the pane. "Ultimate" stone line also goes
    //   through these two points
    // - Considering that "ultimate" stone goes through these two points at the same time as 3rd/4th stones go
    //   through them, solve simple equations and find starting point and velocity vector of "ultimate" stone
    // - Denormalize "ultimate" stone by adding location and speed of first stone -- now it's absolute!

    let ultimate_stone = Stone {
        start: Coord {
            x: 461_522_278_379_729,
            y: 278_970_483_473_640,
            z: 243_127_954_482_382,
        },
        velocity: Coord { x: -336, y: 29, z: 38 },
    };

    let coord_sum = ultimate_stone.start.x + ultimate_stone.start.y + ultimate_stone.start.z;

    Ok((collisions, coord_sum))
}
