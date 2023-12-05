use aoc_common::util;

#[derive(Default, Debug)]
struct CubeSet {
    red: usize,
    blue: usize,
    green: usize,
}

impl CubeSet {
    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.blue >= other.blue && self.green >= other.green
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: usize::max(self.red, other.red),
            blue: usize::max(self.blue, other.blue),
            green: usize::max(self.green, other.green),
        }
    }
}

pub fn play_cube_game(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut id_sum = 0;
    let mut power = 0;
    let basket = CubeSet { red: 12, green: 13, blue: 14 };

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        lexer.literal("Game ")?;
        let id: usize = lexer.unsigned_number()?;
        lexer.literal(": ")?;

        let mut minimal_set = CubeSet::default();
        let mut set = CubeSet::default();

        let mut all_sets_ok = true;

        loop {
            let quantity: usize = lexer.unsigned_number()?;
            lexer.whitespace()?;
            let color_str = lexer.take_while(|ch| ch.is_ascii_alphabetic())?;
            match color_str {
                "red" => set.red = quantity,
                "blue" => set.blue = quantity,
                "green" => set.green = quantity,
                _ => panic!("{}: unknown color {}", line, color_str),
            }

            if lexer.literal(", ").is_ok() {
                continue;
            }

            all_sets_ok = all_sets_ok && basket.contains(&set);
            minimal_set = CubeSet::max(&minimal_set, &set);

            set = CubeSet::default();

            if lexer.literal("; ").is_ok() {
                continue;
            }

            lexer.end()?;
            break;
        }

        if all_sets_ok {
            id_sum += id;
        }

        power += minimal_set.red * minimal_set.blue * minimal_set.green;
    }

    Ok((id_sum, power))
}
