use crate::util::{lexer, Lexer};

use super::data::File;

pub enum ChangeDirArg {
    Down(String),
    Up,
    Root,
}

pub enum Command {
    ChangeDir(ChangeDirArg),
    ListDir,
}

pub fn parse_command(s: &str) -> lexer::Result<Command> {
    let mut lexer = Lexer::of(s);

    lexer.literal("$ ")?;

    if lexer.literal("ls").is_ok() {
        lexer.end()?;
        return Ok(Command::ListDir);
    }

    lexer.literal("cd ")?;

    let arg = if lexer.literal("/").is_ok() {
        ChangeDirArg::Root
    } else if lexer.literal("..").is_ok() {
        ChangeDirArg::Up
    } else {
        ChangeDirArg::Down(lexer.take_rest()?.to_owned())
    };

    lexer.end()?;

    Ok(Command::ChangeDir(arg))
}

pub fn parse_file_entry(s: &str) -> lexer::Result<(String, File)> {
    let mut lexer = Lexer::of(s);

    let file = if lexer.literal("dir").is_ok() {
        File::directory()
    } else {
        let size = lexer.unsigned_number()?;
        File::plain(size)
    };

    lexer.literal(" ")?;
    let name = lexer.take_rest()?;

    Ok((name.to_owned(), file))
}
