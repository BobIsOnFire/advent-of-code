pub trait Processor {
    fn process(&self, value: u64) -> u64;
}

pub enum Operand {
    OldValue,
    Constant(u64),
}

impl Operand {
    pub fn match_old_value(&self, old_value: u64) -> u64 {
        match self {
            Self::OldValue => old_value,
            Self::Constant(num) => *num,
        }
    }
}

pub struct AddOperation {
    first: Operand,
    second: Operand,
}

impl Processor for AddOperation {
    fn process(&self, value: u64) -> u64 {
        self.first.match_old_value(value) + self.second.match_old_value(value)
    }
}

pub struct MulOperation {
    first: Operand,
    second: Operand,
}

impl Processor for MulOperation {
    fn process(&self, value: u64) -> u64 {
        self.first.match_old_value(value) * self.second.match_old_value(value)
    }
}
