mod data;

use aoc_common::util::{
    self,
    lexer::{self, Lexer},
};

use self::data::VideoSystem;

enum Command {
    Add(i64),
    Noop,
}

fn parse_command(s: &str) -> lexer::Result<Command> {
    let mut lexer = Lexer::of(s);

    if lexer.literal("noop").is_ok() {
        lexer.end()?;
        return Ok(Command::Noop);
    }

    lexer.literal("addx ")?;
    let num = lexer.number()?;
    lexer.end()?;

    Ok(Command::Add(num))
}

pub fn get_signal_strengths(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(i64, String)> {
    let mut system = VideoSystem::new(40, 6, 20);

    for line in lines {
        match parse_command(&line)? {
            Command::Noop => system.noop(),
            Command::Add(val) => system.addx(val),
        }
    }

    Ok((
        system.get_total_signal_strength(),
        system.into_screen_render(),
    ))
}
