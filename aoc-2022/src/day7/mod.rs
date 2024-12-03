use std::iter::Peekable;

mod data;
use data::{Directory, File, FileSystem};

mod parser;
use parser::{
    ChangeDirArg::{Down, Root, Up},
    Command::{ChangeDir, ListDir},
};

use aoc_common::util;

fn list_directory<T>(fs: &mut FileSystem, lines: &mut Peekable<T>) -> util::GenericResult<()>
where
    T: Iterator<Item = String>,
{
    loop {
        match lines.next_if(|l| !l.starts_with('$')) {
            None => break Ok(()),
            Some(line) => {
                let (name, file) = parser::parse_file_entry(&line)?;
                fs.create_file(name, file).ok_or_else(|| format!("{line}: File already exists"))?;
            }
        };
    }
}

fn directory_size((_, f): (usize, &File)) -> Option<usize> {
    f.as_directory().map(Directory::get_last_disk_usage)
}

pub fn get_directory_sizes(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut fs = FileSystem::new();
    let mut lines = lines.peekable();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();

        match parser::parse_command(&line)? {
            ChangeDir(Root) => fs.cd_root(),
            ChangeDir(Up) => fs.cd_up(),
            ChangeDir(Down(dir)) => {
                fs.cd_down(&dir).ok_or_else(|| format!("{line}: dir {dir} does not exist"))?;
            }
            ListDir => list_directory(&mut fs, lines.by_ref())?,
        };
    }

    let total_usage = fs.get_total_disk_usage();

    let sum = fs.walk_filesystem().filter_map(directory_size).filter(|&size| size <= 100_000).sum();

    let space_to_free = total_usage - 40_000_000;

    let min_dir = fs
        .walk_filesystem()
        .filter_map(directory_size)
        .filter(|&size| size >= space_to_free)
        .min()
        .ok_or("Cannot find right directory to delete")?;

    Ok((sum, min_dir))
}
