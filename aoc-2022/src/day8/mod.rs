use aoc_common::util;

#[derive(Clone, Copy)]
struct Tree {
    height: i8,
    highest_top: i8,
    highest_bottom: i8,
    highest_left: i8,
    highest_right: i8,
}

impl Tree {
    fn new(height: i8) -> Self {
        Self {
            height,
            highest_top: -1,
            highest_bottom: -1,
            highest_left: -1,
            highest_right: -1,
        }
    }

    fn is_visible(&self) -> bool {
        self.height > self.highest_top
            || self.height > self.highest_bottom
            || self.height > self.highest_left
            || self.height > self.highest_right
    }

    fn set_on_top(&mut self, other: Tree) {
        self.highest_top = Ord::max(other.height, other.highest_top)
    }

    fn set_on_bottom(&mut self, other: Tree) {
        self.highest_bottom = Ord::max(other.height, other.highest_bottom)
    }

    fn set_on_left(&mut self, other: Tree) {
        self.highest_left = Ord::max(other.height, other.highest_left)
    }

    fn set_on_right(&mut self, other: Tree) {
        self.highest_right = Ord::max(other.height, other.highest_right)
    }
}

pub fn find_visible_trees(
    lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut trees: Vec<Vec<Tree>> = lines
        .map(|line| line.bytes().map(|b| Tree::new((b - b'0') as i8)).collect())
        .collect();

    let (width, height) = (trees[0].len(), trees.len());
    for row in 1..height {
        for col in 1..width {
            let (top, left) = (trees[row - 1][col], trees[row][col - 1]);
            trees[row][col].set_on_top(top);
            trees[row][col].set_on_left(left);
        }
    }

    let (width, height) = (trees[0].len(), trees.len());
    for row in (0..(height - 1)).rev() {
        for col in (0..(width - 1)).rev() {
            let (bottom, right) = (trees[row + 1][col], trees[row][col + 1]);
            trees[row][col].set_on_bottom(bottom);
            trees[row][col].set_on_right(right);
        }
    }

    let visible = trees
        .iter()
        .map(|row| row.iter().filter(|tree| tree.is_visible()).count())
        .sum();

    Ok((visible, 0))
}
