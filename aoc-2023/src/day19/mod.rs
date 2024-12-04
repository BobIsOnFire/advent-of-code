use std::collections::HashMap;

use aoc_common::util;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    Extreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::Extreme,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => panic!("Unknown category {value}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Less,
    Greater,
}

impl From<char> for Operation {
    fn from(value: char) -> Self {
        match value {
            '<' => Self::Less,
            '>' => Self::Greater,
            _ => panic!("Unknown operation {value}"),
        }
    }
}

#[derive(Debug)]
enum Transition {
    Accept,
    Reject,
    Next(String),
}

impl From<&str> for Transition {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            other => Self::Next(other.to_owned()),
        }
    }
}

struct Detail {
    rating: [usize; 4],
}

impl Detail {
    const fn new() -> Self {
        Self { rating: [0; 4] }
    }

    const fn get_rating(&self, category: Category) -> &usize {
        &self.rating[category as usize]
    }

    fn get_rating_mut(&mut self, category: Category) -> &mut usize {
        &mut self.rating[category as usize]
    }

    fn total_rank(&self) -> usize {
        self.rating.iter().sum()
    }
}

#[derive(Clone, Debug)]
struct DetailRange {
    rating_ranges: [(usize, usize); 4],
}

impl DetailRange {
    const fn new() -> Self {
        Self { rating_ranges: [(1, 4000); 4] }
    }

    const fn get_rating_range(&self, category: Category) -> &(usize, usize) {
        &self.rating_ranges[category as usize]
    }

    fn get_rating_range_mut(&mut self, category: Category) -> &mut (usize, usize) {
        &mut self.rating_ranges[category as usize]
    }

    fn size(&self) -> usize {
        self.rating_ranges
            .iter()
            .map(|(from, to)| (to + 1).saturating_sub(*from))
            .product()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

struct Condition {
    category: Category,
    operation: Operation,
    value: usize,
}

impl Condition {
    const fn check(&self, detail: &Detail) -> bool {
        let rank = *detail.get_rating(self.category);

        match self.operation {
            Operation::Less => rank < self.value,
            Operation::Greater => rank > self.value,
        }
    }

    fn split(&self, detail_range: &DetailRange) -> (DetailRange, DetailRange) {
        let (from, to) = *detail_range.get_rating_range(self.category);

        let mut less_range = detail_range.clone();
        let mut greater_range = detail_range.clone();

        match self.operation {
            Operation::Less => {
                *less_range.get_rating_range_mut(self.category) = (from, self.value - 1);
                *greater_range.get_rating_range_mut(self.category) = (self.value, to);
                (less_range, greater_range)
            }
            Operation::Greater => {
                *less_range.get_rating_range_mut(self.category) = (from, self.value);
                *greater_range.get_rating_range_mut(self.category) = (self.value + 1, to);
                (greater_range, less_range)
            }
        }
    }
}

struct Rule {
    condition: Option<Condition>,
    transition: Transition,
}

impl Rule {
    fn get_transition(&self, detail: &Detail) -> Option<&Transition> {
        self.condition
            .as_ref()
            .map_or(Some(&self.transition), |cond| {
                cond.check(detail).then_some(&self.transition)
            })
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn get_transition(&self, detail: &Detail) -> &Transition {
        self.rules
            .iter()
            .find_map(|rule| rule.get_transition(detail))
            .expect("There should be at least one transition available")
    }

    fn all_transitions(&self, mut detail_range: DetailRange) -> Vec<(DetailRange, &Transition)> {
        let mut result = vec![];
        for rule in &self.rules {
            if let Some(cond) = &rule.condition {
                let (if_true, if_false) = cond.split(&detail_range);

                if !if_true.is_empty() {
                    result.push((if_true, &rule.transition));
                }

                if if_false.is_empty() {
                    break;
                }

                detail_range = if_false;
            } else {
                result.push((detail_range, &rule.transition));
                break;
            }
        }

        result
    }
}

fn do_accepted_range_size(
    workflows: &HashMap<String, Workflow>,
    current: &Workflow,
    range: DetailRange,
) -> usize {
    let mut total = 0;
    for (next_range, transition) in current.all_transitions(range) {
        match transition {
            Transition::Accept => total += next_range.size(),
            Transition::Reject => {}
            Transition::Next(next) => {
                total += do_accepted_range_size(
                    workflows,
                    workflows
                        .get(next)
                        .expect("All workflows should be defined"),
                    next_range,
                );
            }
        }
    }
    total
}

pub fn find_ratings(
    mut lines: impl Iterator<Item = String>,
) -> util::GenericResult<(usize, usize)> {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        let mut lexer = util::Lexer::of(&line);
        let label = lexer.before_literal("{")?.to_owned();

        let rules_str = lexer.before_literal("}")?;
        lexer.end()?;

        let mut rules = vec![];
        for rule_str in rules_str.split(',') {
            let rule = if rule_str.contains(':') {
                let mut lexer = util::Lexer::of(rule_str);

                let condition = Condition {
                    category: Category::from(lexer.symbol()?),
                    operation: Operation::from(lexer.symbol()?),
                    value: lexer.unsigned_number()?,
                };

                lexer.literal(":")?;

                let transition = Transition::from(lexer.take_rest()?);

                Rule {
                    condition: Some(condition),
                    transition,
                }
            } else {
                Rule {
                    condition: None,
                    transition: Transition::from(rule_str),
                }
            };

            rules.push(rule);
        }

        workflows.insert(label, Workflow { rules });
    }

    let mut details = vec![];
    for line in lines {
        let mut lexer = util::Lexer::of(&line);
        lexer.literal("{")?;
        let detail_str = lexer.before_literal("}")?;
        lexer.end()?;

        let mut detail = Detail::new();
        for rank_str in detail_str.split(',') {
            let mut lexer = util::Lexer::of(rank_str);
            let category = Category::from(lexer.symbol()?);
            lexer.literal("=")?;
            let rank = lexer.unsigned_number()?;
            lexer.end()?;

            *detail.get_rating_mut(category) = rank;
        }

        details.push(detail);
    }

    let mut accepted_sum = 0;
    for detail in details {
        let mut current_workflow = workflows
            .get("in")
            .expect("All workflows should be defined");

        loop {
            match current_workflow.get_transition(&detail) {
                Transition::Next(next) => {
                    current_workflow = workflows
                        .get(next)
                        .expect("All workflows should be defined");
                }
                Transition::Accept => {
                    accepted_sum += detail.total_rank();
                    break;
                }
                Transition::Reject => break,
            }
        }
    }

    let total_size = do_accepted_range_size(
        &workflows,
        workflows
            .get("in")
            .expect("All workflows should be defined"),
        DetailRange::new(),
    );

    Ok((accepted_sum, total_size))
}
