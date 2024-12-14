use aoc_common::util::{self, MatrixIndex, VecMatrix};

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

struct Robot {
    start: (usize, usize),
    velocity: (usize, usize),
}

impl Robot {
    const fn new(start: (usize, usize), velocity: (isize, isize)) -> Self {
        let velocity = (
            if velocity.0 < 0 {
                WIDTH - velocity.0.unsigned_abs()
            } else {
                velocity.0 as usize
            },
            if velocity.1 < 0 {
                HEIGHT - velocity.1.unsigned_abs()
            } else {
                velocity.1 as usize
            },
        );

        Self { start, velocity }
    }

    const fn get_position(&self, seconds: usize) -> (usize, usize) {
        (
            (self.start.0 + seconds * self.velocity.0) % WIDTH,
            (self.start.1 + seconds * self.velocity.1) % HEIGHT,
        )
    }
}

#[allow(unused)]
fn draw(robots: &[Robot], seconds: usize) {
    let mut matrix = VecMatrix::with_data(vec![0; WIDTH * HEIGHT], WIDTH);

    for robot in robots {
        let (x, y) = robot.get_position(seconds);
        matrix[MatrixIndex { row: y, col: x }] += 1;
    }

    for (idx, &count) in matrix.iter_enumerate() {
        if count == 0 {
            print!(".");
        } else {
            print!("{count}");
        }

        if idx.col == matrix.width() - 1 {
            println!();
        }
    }
    println!("SECONDS: {seconds}");
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut robots = vec![];

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        lexer.literal("p=")?;
        let start_x: usize = lexer.unsigned_number()?;
        lexer.literal(",")?;
        let start_y: usize = lexer.unsigned_number()?;

        lexer.literal(" v=")?;
        let vel_x: isize = lexer.number()?;
        lexer.literal(",")?;
        let vel_y: isize = lexer.number()?;
        lexer.end()?;

        robots.push(Robot::new((start_x, start_y), (vel_x, vel_y)));
    }

    let mut quadrants = [0usize; 4];
    for robot in &robots {
        let (x, y) = robot.get_position(100);
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue;
        }

        let is_left = (0..(WIDTH / 2)).contains(&x);
        let is_top = (0..(HEIGHT / 2)).contains(&y);

        match (is_left, is_top) {
            (false, false) => quadrants[0] += 1,
            (false, true) => quadrants[1] += 1,
            (true, false) => quadrants[2] += 1,
            (true, true) => quadrants[3] += 1,
        }
    }

    let safety_factor = quadrants.into_iter().product();

    // for seconds in 0..10000 {
    //     draw(&robots, seconds);
    // }

    // Determined by saving 10000 pictures into the file and searching for long continuous robot line
    let easter_egg_seconds = 6888;

    Ok((safety_factor, easter_egg_seconds))
}
