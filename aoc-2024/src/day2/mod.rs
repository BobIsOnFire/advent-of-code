use aoc_common::util;

struct ReportView<'a> {
    numbers: &'a [u64],
    skipped_idx: Option<usize>,
}

impl<'a> ReportView<'a> {
    const fn new_strict(numbers: &'a [u64]) -> Self {
        Self { numbers, skipped_idx: None }
    }

    const fn new_dampened(numbers: &'a [u64], idx: usize) -> Self {
        Self { numbers, skipped_idx: Some(idx) }
    }

    const fn len(&self) -> usize {
        if self.skipped_idx.is_some() {
            self.numbers.len() - 1
        } else {
            self.numbers.len()
        }
    }

    const fn get_nth_pair(&self, n: usize) -> (u64, u64) {
        let (first, second) = if let Some(idx) = self.skipped_idx {
            if idx <= n {
                (n + 1, n + 2)
            } else if idx == n + 1 {
                (n, n + 2)
            } else {
                (n, n + 1)
            }
        } else {
            (n, n + 1)
        };

        (self.numbers[first], self.numbers[second])
    }
}

fn check_view_safe(view: &ReportView) -> bool {
    let (first, second) = view.get_nth_pair(0);
    let increasing = first < second;

    for i in 0..(view.len() - 1) {
        let (first, second) = view.get_nth_pair(i);

        if first == second {
            return false;
        }

        if increasing != (first < second) {
            return false;
        }

        if u64::abs_diff(first, second) > 3 {
            return false;
        }
    }

    true
}

fn check_safe_strict(numbers: &[u64]) -> bool {
    check_view_safe(&ReportView::new_strict(numbers))
}

fn check_safe_dampened(numbers: &[u64]) -> bool {
    for i in 0..numbers.len() {
        if check_view_safe(&ReportView::new_dampened(numbers, i)) {
            return true;
        }
    }
    false
}

pub fn count_safe_systems(lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let mut safe_counter_strict = 0;
    let mut safe_counter_dampened = 0;

    for line in lines {
        let numbers = line.split_ascii_whitespace().map(str::parse).collect::<Result<Vec<u64>, _>>()?;

        if check_safe_strict(&numbers) {
            safe_counter_strict += 1;
        } else if check_safe_dampened(&numbers) {
            safe_counter_dampened += 1;
        }
    }

    Ok((safe_counter_strict, safe_counter_strict + safe_counter_dampened))
}
