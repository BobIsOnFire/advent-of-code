use std::{cell::RefCell, collections::HashMap};

use aoc_common::util;

type Id = usize;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Job {
    Number(i64),
    Operation { id1: Id, id2: Id, op: Operation },
}

#[allow(unused)]
impl Job {
    fn get_result(&self, first: i64, second: i64) -> i64 {
        match self {
            Self::Number(num) => *num,
            Self::Operation { id1: _, id2: _, op } => match op {
                Operation::Add => first + second,
                Operation::Sub => first - second,
                Operation::Multiply => first * second,
                Operation::Divide => first / second,
            },
        }
    }

    fn get_result_by(&self, func: impl Fn(Id) -> i64) -> i64 {
        match self {
            Self::Number(num) => *num,
            Self::Operation { id1, id2, op } => {
                let first = func(*id1);
                let second = func(*id2);
                match op {
                    Operation::Add => first + second,
                    Operation::Sub => first - second,
                    Operation::Multiply => first * second,
                    Operation::Divide => first / second,
                }
            }
        }
    }

    fn get_first_operand(&self, second: i64, result: i64) -> i64 {
        match self {
            Self::Number(num) => *num,
            Self::Operation { id1: _, id2: _, op } => match op {
                Operation::Add => result - second,
                Operation::Sub => result + second,
                Operation::Multiply => result / second,
                Operation::Divide => result * second,
            },
        }
    }

    fn get_second_operand(&self, first: i64, result: i64) -> i64 {
        match self {
            Self::Number(num) => *num,
            Self::Operation { id1: _, id2: _, op } => match op {
                Operation::Add => result - first,
                Operation::Sub => first - result,
                Operation::Multiply => result / first,
                Operation::Divide => first / result,
            },
        }
    }

    fn check_depends(&self, check_id: Id, mut id_to_dep: impl FnMut(Id, Id) -> bool) -> bool {
        match self {
            Self::Number(_) => false,
            Self::Operation { id1, id2, op: _ } => *id1 == check_id || *id2 == check_id || id_to_dep(*id1, check_id) || id_to_dep(*id2, check_id),
        }
    }
}

struct MonkeySet {
    name_to_id: HashMap<String, Id>,
    jobs: Vec<Option<Job>>,
    operation_cache: RefCell<HashMap<Id, i64>>,
    depends_cache: RefCell<HashMap<Id, bool>>,
}

impl MonkeySet {
    fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            jobs: Vec::new(),
            operation_cache: RefCell::new(HashMap::new()),
            depends_cache: RefCell::new(HashMap::new()),
        }
    }

    fn get_or_create_monkey(&mut self, name: &str) -> Id {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }

        let id = self.jobs.len();
        self.name_to_id.insert(name.to_owned(), id);
        self.jobs.push(None);

        id
    }

    fn add_monkey_job(&mut self, name: &str, job: Job) {
        let id = self.get_or_create_monkey(name);
        if self.jobs[id].is_some() {
            panic!("Cannot insert one monkey with two jobs: {}", name);
        }
        self.jobs[id] = Some(job);
    }

    fn get_job_by_id(&self, id: Id) -> &Job {
        self.jobs[id].as_ref().expect("All monkeys should have a job defined!")
    }

    fn get_monkey_id(&self, name: &str) -> Id {
        *self.name_to_id.get(name).unwrap_or_else(|| panic!("{} not found", name))
    }

    fn get_result_by_id(&self, id: Id) -> i64 {
        let cached_result = self.operation_cache.borrow().get(&id).copied();
        if let Some(num) = cached_result {
            num
        } else {
            let num = self.get_job_by_id(id).get_result_by(|id| self.get_result_by_id(id));
            self.operation_cache.borrow_mut().insert(id, num);
            num
        }
    }

    fn check_depends_by_id(&self, id: Id, check_id: Id) -> bool {
        if id == check_id {
            true
        } else {
            let cached_depends = self.depends_cache.borrow().get(&id).copied();
            if let Some(depends) = cached_depends {
                depends
            } else {
                let depends = self
                    .get_job_by_id(id)
                    .check_depends(check_id, |id, check_id| self.check_depends_by_id(id, check_id));
                self.depends_cache.borrow_mut().insert(id, depends);
                depends
            }
        }
    }

    fn get_operand_by_id(&self, id: Id, operand_id: Id, result: i64) -> i64 {
        if id == operand_id {
            result
        } else {
            if !self.check_depends_by_id(id, operand_id) {
                panic!(
                    "Current id ({}) doesn't depend on operand that we are searching ({})! Are we lost?",
                    id, operand_id
                );
            }

            let job = self.get_job_by_id(id);

            if let Job::Operation { id1, id2, op: _ } = job {
                if self.check_depends_by_id(*id1, operand_id) {
                    let result = job.get_first_operand(self.get_result_by_id(*id2), result);
                    self.get_operand_by_id(*id1, operand_id, result)
                } else {
                    let result = job.get_second_operand(self.get_result_by_id(*id1), result);
                    self.get_operand_by_id(*id2, operand_id, result)
                }
            } else {
                unreachable!("Job::Number always returns `false` for check_depends, which I already checked")
            }
        }
    }

    fn get_result(&self, name: &str) -> i64 {
        self.get_result_by_id(self.get_monkey_id(name))
    }

    fn get_operand(&self, root_name: &str, operand_name: &str) -> i64 {
        let root_id = self.get_monkey_id(root_name);
        let operand_id = self.get_monkey_id(operand_name);

        let root_job = self.get_job_by_id(root_id);

        if let Job::Operation { id1, id2, op: _ } = root_job {
            if self.check_depends_by_id(*id1, operand_id) {
                self.get_operand_by_id(*id1, operand_id, self.get_result_by_id(*id2))
            } else {
                self.get_operand_by_id(*id2, operand_id, self.get_result_by_id(*id1))
            }
        } else {
            0 // root_name doesn't depend on operand_name
        }
    }
}

fn parse_operation_sign(sign: &str) -> util::GenericResult<Operation> {
    match sign {
        "+" => Ok(Operation::Add),
        "-" => Ok(Operation::Sub),
        "*" => Ok(Operation::Multiply),
        "/" => Ok(Operation::Divide),
        _ => Err(format!("Unknown operation {}", sign).into()),
    }
}

pub fn get_answer(lines: impl Iterator<Item = String>) -> util::GenericResult<(i64, i64)> {
    let mut monkey_set = MonkeySet::new();

    for line in lines {
        let mut lexer = util::Lexer::of(&line);

        let name = lexer.before_literal(": ")?;
        let job = if let Ok(num) = lexer.number() {
            Job::Number(num)
        } else {
            let operand1 = lexer.before_literal(" ")?;
            let sign = lexer.before_literal(" ")?;
            let operand2 = lexer.take_rest()?;

            let id1 = monkey_set.get_or_create_monkey(operand1);
            let id2 = monkey_set.get_or_create_monkey(operand2);

            let op = parse_operation_sign(sign)?;

            Job::Operation { id1, id2, op }
        };

        monkey_set.add_monkey_job(name, job);
    }

    let root_num = monkey_set.get_result("root");
    let humn_num = monkey_set.get_operand("root", "humn");

    Ok((root_num, humn_num))
}
