use aoc_common::util;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Range {
    start: usize,
    len: usize,
}

impl Range {
    const fn new(start: usize, len: usize) -> Self {
        Self { start, len }
    }

    const fn from_bounds(start: usize, end: usize) -> Self {
        Self { start, len: end - start + 1 }
    }

    const fn start(&self) -> usize {
        self.start
    }

    const fn len(&self) -> usize {
        self.len
    }

    const fn end(&self) -> usize {
        self.start + self.len - 1
    }

    fn contains(&self, value: usize) -> bool {
        (self.start()..=self.end()).contains(&value)
    }

    fn intersects(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end()) || other.contains(self.start()) || other.contains(self.end())
    }

    const fn is_followed_by(&self, other: &Self) -> bool {
        other.start() == self.end() + 1
    }

    fn get_union(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) || self.is_followed_by(other) || other.is_followed_by(self) {
            let start = usize::min(self.start(), other.start());
            let end = usize::max(self.end(), other.end());
            Some(Self::from_bounds(start, end))
        } else {
            None
        }
    }

    fn get_intersection(&self, other: &Self) -> Option<Self> {
        if self.intersects(other) {
            let start = usize::max(self.start(), other.start());
            let end = usize::min(self.end(), other.end());
            Some(Self::from_bounds(start, end))
        } else {
            None
        }
    }

    const fn get_difference(&self, other: &Self) -> (Option<Self>, Option<Self>) {
        let left = if self.start() < other.start() {
            Some(Self::from_bounds(self.start(), other.start() - 1))
        } else {
            None
        };

        let right = if self.end() > other.end() {
            Some(Self::from_bounds(other.end() + 1, self.end()))
        } else {
            None
        };

        (left, right)
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}; {}]", self.start(), self.end())
    }
}

struct IntegerSet {
    ranges: Vec<Range>,
}

impl IntegerSet {
    const fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn insert(&mut self, mut range: Range) {
        let mut new_ranges = Vec::with_capacity(self.ranges.len());

        for r in self.ranges.drain(..) {
            if let Some(merged) = range.get_union(&r) {
                range = merged;
            } else {
                new_ranges.push(r);
            }
        }

        new_ranges.push(range);
        self.ranges = new_ranges;
    }

    fn remove(&mut self, range: &Range) {
        let mut new_ranges = Vec::with_capacity(self.ranges.len());

        for r in self.ranges.drain(..) {
            if range.intersects(&r) {
                let (left, right) = r.get_difference(range);
                if let Some(left_range) = left {
                    new_ranges.push(left_range);
                }
                if let Some(right_range) = right {
                    new_ranges.push(right_range);
                }
            } else {
                new_ranges.push(r);
            }
        }

        self.ranges = new_ranges;
    }

    #[allow(dead_code)]
    fn print(&self, prefix: &str) {
        let mut ranges = self.ranges.clone();
        ranges.sort_by_key(Range::start);
        let strings = ranges.iter().map(|r| format!("{r}")).collect::<Vec<_>>();

        println!("{}: [{}]", prefix, strings.join(", "));
    }

    fn iter_ranges(&self) -> impl Iterator<Item = &Range> {
        self.ranges.iter()
    }

    fn into_iter_ranges(self) -> impl Iterator<Item = Range> {
        self.ranges.into_iter()
    }
}

fn parse_seeds(line: &str) -> util::lexer::Result<(Vec<usize>, IntegerSet)> {
    let mut lexer = util::Lexer::of(line);
    lexer.literal("seeds:")?;

    let mut values = Vec::new();
    let mut ranges = IntegerSet::new();

    while lexer.end().is_err() {
        lexer.whitespace()?;
        let val1 = lexer.unsigned_number()?;
        lexer.whitespace()?;
        let val2 = lexer.unsigned_number()?;

        values.extend([val1, val2]);
        ranges.insert(Range::new(val1, val2));
    }

    Ok((values, ranges))
}

pub fn find_locations(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let (mut values, mut ranges) = parse_seeds(&lines.next().expect("Seed line is expected"))?;

    let _ = lines.next().expect("Input ended too soon");

    let maps = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    for map in maps {
        util::Lexer::of(&lines.next().expect("Input ended too soon"))
            .chain()
            .literal(map)?
            .literal(" map:")?
            .end()?;

        let mut new_values = values.clone();
        let mut new_ranges = IntegerSet::new();

        loop {
            let data = match lines.next() {
                None => break,
                Some(s) if s.is_empty() => break,
                Some(s) => s,
            };
            let mut lexer = util::Lexer::of(&data);

            let dest_start = lexer.unsigned_number::<usize>()?;
            lexer.whitespace()?;
            let source_start = lexer.unsigned_number::<usize>()?;
            lexer.whitespace()?;
            let range_len = lexer.unsigned_number::<usize>()?;
            lexer.whitespace()?;

            let source_range = Range::new(source_start, range_len);
            let dest_range = Range::new(dest_start, range_len);

            for (i, value) in values.iter().enumerate() {
                if source_range.contains(*value) {
                    new_values[i] = dest_range.start() + (value - source_range.start());
                }
            }

            for range in ranges.iter_ranges() {
                if let Some(source_intersect) = source_range.get_intersection(range) {
                    let dest_intersect = Range::new(
                        dest_range.start() + (source_intersect.start() - source_range.start()),
                        source_intersect.len(),
                    );
                    new_ranges.insert(dest_intersect);
                }
            }
            ranges.remove(&source_range);
        }

        values = new_values;

        for range in new_ranges.into_iter_ranges() {
            ranges.insert(range);
        }
    }

    let min_location = values.into_iter().min().expect("There should be at least one location");
    let min_range = ranges
        .iter_ranges()
        .map(Range::start)
        .min()
        .expect("There should be at least one location");

    Ok((min_location, min_range))
}
