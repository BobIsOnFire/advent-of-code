use std::collections::HashMap;

use aoc_common::util::{self, iter::ResultIteratorExtended};

enum Action {
    Remove,
    Update(u8),
}

struct Operation {
    label: String,
    action: Action,
}

#[derive(Clone)]
struct LenseBox {
    lenses: Vec<u8>,
    label_to_pos: HashMap<String, usize>,
}

impl LenseBox {
    fn new() -> Self {
        Self {
            lenses: Vec::new(),
            label_to_pos: HashMap::new(),
        }
    }

    fn get_or_insert(&mut self, label: String) -> usize {
        if let Some(pos) = self.label_to_pos.get(&label) {
            *pos
        } else {
            self.lenses.push(0);
            self.label_to_pos.insert(label, self.lenses.len() - 1);
            self.lenses.len() - 1
        }
    }

    fn update_lense(&mut self, label: String, value: u8) {
        let pos = self.get_or_insert(label);
        self.lenses[pos] = value;
    }

    fn remove_lense(&mut self, label: String) {
        if let Some(pos) = self.label_to_pos.remove(&label) {
            self.lenses.remove(pos);
            for other in self.label_to_pos.values_mut() {
                if *other > pos {
                    *other -= 1;
                }
            }
        }
    }

    fn process(&mut self, operation: Operation) {
        match operation.action {
            Action::Update(value) => self.update_lense(operation.label, value),
            Action::Remove => self.remove_lense(operation.label),
        }
    }

    fn get_total_power(&self) -> usize {
        self.lenses.iter().enumerate().map(|(idx, &value)| (idx + 1) * value as usize).sum()
    }
}

fn parse_operation(s: &str) -> util::lexer::Result<Operation> {
    let mut lexer = util::Lexer::of(s);
    let label = lexer.take_while(|ch| ch.is_ascii_alphabetic())?.to_owned();

    let action = match lexer.symbol()? {
        '=' => Action::Update(lexer.unsigned_number()?),
        '-' => Action::Remove,
        ch => panic!("Unknown operation: {}", ch),
    };

    lexer.end()?;

    Ok(Operation { label, action })
}

fn get_hash(s: &str) -> u8 {
    s.bytes().fold(0, |acc, byte| acc.wrapping_add(byte).wrapping_mul(17))
}

pub fn focus_lenses(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(usize, usize)> {
    let line = lines.next().expect("Input should not be empty");
    let hash_sum = line.split(',').map(get_hash).map(usize::from).sum();

    let mut boxes = vec![LenseBox::new(); 256];

    let mut operations = line.split(',').map(parse_operation).end_on_error();

    operations.by_ref().for_each(|op| boxes[get_hash(&op.label) as usize].process(op));
    operations.into_err()?;

    let total_lense_sum = boxes
        .into_iter()
        .enumerate()
        .map(|(idx, lense_box)| (idx + 1) * lense_box.get_total_power())
        .sum();

    Ok((hash_sum, total_lense_sum))
}
