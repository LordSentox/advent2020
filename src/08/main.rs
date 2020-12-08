use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Opcode {
    // No operation
    Noop,
    // Jump relative to the current instruction
    Jump,
    // Change global accumulator by a given value
    Acc,
}

struct Operation {
    code: Opcode,
    val: isize,
}

#[derive(thiserror::Error, Debug)]
enum OperationParseError {
    #[error("{0} is not a known operational code")]
    UnknownOpcode(String),
    #[error("instruction must have {0} parts, {1} were supplied")]
    IncorrectLength(usize, usize),
    #[error("{1} does not have the correct format. Expected {0}")]
    WrongArgumentFormat(String, String),
}

impl FromStr for Opcode {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Self::Noop),
            "jmp" => Ok(Self::Jump),
            "acc" => Ok(Self::Acc),
            other => Err(OperationParseError::UnknownOpcode(other.to_owned())),
        }
    }
}

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation: Vec<&str> = s.split_whitespace().collect();
        if operation.len() != 2 {
            return Err(OperationParseError::IncorrectLength(2, operation.len()));
        }

        let code = Opcode::from_str(operation[0])?;
        let val = match operation[1].parse::<isize>() {
            Ok(val) => val,
            Err(_) => {
                return Err(OperationParseError::WrongArgumentFormat(
                    "isize".to_string(),
                    operation[1].to_owned(),
                ))
            }
        };

        Ok(Self { code, val })
    }
}

fn main() {
    let input = fs::read_to_string("input/08").expect("Unable to open input file");
    let operations: Vec<Operation> = input
        .lines()
        .map(|op| Operation::from_str(op).expect("Unable to parse operation"))
        .collect();

    let mut visited = vec![false; operations.len()];

    let mut acc = 0;
    let mut pc = 0;
    loop {
        // Check if this operation has been executed before. Since the program is non-branching
        // this means we have found an infinite loop.
        if visited[pc] {
            println!("Found infinite loop on instruction {} -- aborting", pc);
            break;
        }
        visited[pc] = true;

        // Perform the operation.
        match operations[pc] {
            Operation {
                code: Opcode::Acc,
                val,
            } => {
                acc += val;
                pc += 1;
            }
            Operation {
                code: Opcode::Noop, ..
            } => {
                pc += 1;
            }
            Operation {
                code: Opcode::Jump,
                val,
            } => pc = (pc as isize + val) as usize,
        }
    }

    println!("Accumulator value for a) {}", acc);
}
