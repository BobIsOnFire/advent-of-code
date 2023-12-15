use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::util::{self, iter::ResultIteratorExtended};

pub struct Solution<T1, T2, F>
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
    F: Fn(&mut dyn Iterator<Item = String>) -> util::GenericResult<(T1, T2)>,
{
    title: String,
    processor: Option<F>,
}

impl<T1, T2, F> Solution<T1, T2, F>
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
    F: Fn(&mut dyn Iterator<Item = String>) -> util::GenericResult<(T1, T2)>,
{
    pub fn new(title: impl Into<String>) -> Self {
        Self { title: title.into(), processor: None }
    }

    pub fn solution(mut self, func: F) -> Self {
        self.processor = Some(func);
        self
    }

    fn do_run(self, input: impl AsRef<std::path::Path>) -> util::GenericResult<()> {
        println!();
        println!("{}", self.title);
        println!();

        let reader = BufReader::new(File::open(input)?);
        let mut iter = reader.lines().end_on_error();

        let (ans1, ans2) = self.processor.ok_or("Processor is None")?(&mut iter)?;
        iter.into_err()?;

        println!("--- Part 1 ---");
        println!();
        println!("{}", ans1);
        println!();
        println!("--- Part 2 ---");
        println!();
        println!("{}", ans2);
        println!();

        Ok(())
    }

    pub fn run(self, input: impl AsRef<std::path::Path>) {
        let result = self.do_run(input);
        if let Err(e) = result {
            eprintln!("{:#?}", e);
        }
    }
}
