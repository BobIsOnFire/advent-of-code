use std::mem;

use aoc_common::util;

#[allow(dead_code)] // linter warns on fields, we use them only to print in Debug/Display impl
#[derive(Debug)]
pub struct ByteParseError {
    expected: String,
    actual: u8,
}

impl std::fmt::Display for ByteParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ByteParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ComboOperand {
    Literal(u8),
    Register(u8),
}

impl TryFrom<u8> for ComboOperand {
    type Error = ByteParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => Ok(Self::Literal(value)),
            4..=6 => Ok(Self::Register(value - 4)),
            b => Err(ByteParseError {
                expected: "Combo operand code (0-6)".to_string(),
                actual: b,
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Adv(ComboOperand),
    Bxl(u8),
    Bst(ComboOperand),
    Jnz(u8),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl TryFrom<(u8, u8)> for Operation {
    type Error = ByteParseError;

    fn try_from((instr, op): (u8, u8)) -> Result<Self, Self::Error> {
        match instr {
            0 => Ok(Self::Adv(op.try_into()?)),
            1 => Ok(Self::Bxl(op)),
            2 => Ok(Self::Bst(op.try_into()?)),
            3 => Ok(Self::Jnz(op)),
            4 => Ok(Self::Bxc),
            5 => Ok(Self::Out(op.try_into()?)),
            6 => Ok(Self::Bdv(op.try_into()?)),
            7 => Ok(Self::Cdv(op.try_into()?)),
            b => Err(ByteParseError {
                expected: "Operation (two 0-7 numbers)".to_string(),
                actual: b,
            }),
        }
    }
}

struct Processor {
    registers: [u64; 3],
    program: Vec<Operation>,
    isp: usize,
}

impl Processor {
    const fn new(registers: [u64; 3], program: Vec<Operation>) -> Self {
        Self { registers, program, isp: 0 }
    }

    fn get_value(&self, operand: ComboOperand) -> u64 {
        match operand {
            ComboOperand::Literal(value) => value.into(),
            ComboOperand::Register(reg) => self.registers[reg as usize],
        }
    }

    fn process_operation(&mut self, output: &mut Vec<u8>, operation: Operation) {
        self.isp += 1;

        match operation {
            Operation::Adv(combo) => self.registers[0] /= 2u64.pow(self.get_value(combo) as u32),
            Operation::Bxl(value) => self.registers[1] ^= u64::from(value),
            Operation::Bst(combo) => self.registers[1] = self.get_value(combo) % 8,
            Operation::Jnz(value) => {
                if self.registers[0] != 0 {
                    self.isp = value as usize;
                }
            }
            Operation::Bxc => self.registers[1] ^= self.registers[2],
            Operation::Out(combo) => output.push((self.get_value(combo) % 8) as u8),
            Operation::Bdv(combo) => {
                self.registers[1] = self.registers[0] / 2u64.pow(self.get_value(combo) as u32);
            }
            Operation::Cdv(combo) => {
                self.registers[2] = self.registers[0] / 2u64.pow(self.get_value(combo) as u32);
            }
        }
    }

    fn run_program(&mut self) -> Vec<u8> {
        let mut output = vec![];
        self.isp = 0;

        while let Some(operation) = self.program.get(self.isp) {
            // println!(
            //     "Regs: {:?}, isp: {}, instr: {:?}",
            //     self.registers, self.isp, operation
            // );
            self.process_operation(&mut output, *operation);
        }

        // println!("Regs: {:?}, isp: {}", self.registers, self.isp);

        output
    }
}

fn parse_register(line: &str, prefix: &str) -> util::lexer::Result<u64> {
    let mut lexer = util::Lexer::of(line);
    lexer.literal(prefix)?;
    lexer.literal(": ")?;
    let num = lexer.unsigned_number()?;
    lexer.end()?;
    Ok(num)
}

pub fn get_answer(mut lines: impl Iterator<Item = String>) -> util::GenericResult<(String, u64)> {
    let registers = [
        parse_register(&lines.next().ok_or("Input ended too early")?, "Register A")?,
        parse_register(&lines.next().ok_or("Input ended too early")?, "Register B")?,
        parse_register(&lines.next().ok_or("Input ended too early")?, "Register C")?,
    ];

    lines.next().ok_or("Input ended too early")?;

    let program_line = lines.next().ok_or("Input ended too early")?;
    let mut lexer = util::Lexer::of(&program_line);
    lexer.literal("Program: ")?;

    let mut recursive_output = vec![];
    let mut program = vec![];
    loop {
        let instruction = lexer.unsigned_number::<u8>()?;
        lexer.literal(",")?;
        let operand = lexer.unsigned_number::<u8>()?;
        program.push((instruction, operand).try_into()?);
        recursive_output.extend([instruction, operand]);

        if lexer.literal(",").is_err() {
            break;
        }
    }
    lexer.end()?;

    let mut processor = Processor::new(registers, program.clone());
    let output = processor.run_program();
    let output_len = output.len();

    let output_bytes = output
        .into_iter()
        .flat_map(|b| [b + b'0', b','])
        .take(output_len * 2 - 1)
        .collect::<Vec<_>>();

    let output_str = String::from_utf8(output_bytes)?;

    // Assume:
    // - last instruction is jnz
    // - last instruction on A register is adv 3 (A is divided by 8 before entering next iteration)
    // - Values of B or C from previous iteration are not used in next iteration
    // - program outputs exactly one value in a single iteration

    let mut valid_a = vec![0];

    let mut processor = Processor::new([0, 0, 0], program.clone());
    processor.program.pop();

    for out in recursive_output.iter().rev().copied() {
        for check_a in mem::take(&mut valid_a) {
            for a in (check_a * 8)..(check_a * 8 + 8) {
                processor.registers[0] = a;
                let output = processor.run_program();
                if output[0] == out {
                    valid_a.push(a);
                }
            }
        }
    }

    let result_a = valid_a[0];
    assert!(recursive_output == Processor::new([result_a, 0, 0], program).run_program());

    Ok((output_str, result_a))
}
