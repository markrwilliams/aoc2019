use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::result;
use std::iter;

pub type AllOutputResult = std::result::Result<Vec<i64>, IntcodeError>;
pub type OutputResult = std::result::Result<Option<i64>, IntcodeError>;

#[derive(Debug)]
pub enum IntcodeError {
    UnknownError,
    UnknownOpcode(usize, i64),
    InvalidParameterType(usize, i64, &'static str),
    UnknownParameterType(usize, i64),
    NegativePosition(usize, i64, i64),
    MissingInput(usize),
}

impl fmt::Display for IntcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IntcodeError::*;
        match self {
            UnknownError => write!(f, "unknown error"),
            UnknownOpcode(pc, o) => write!(f, "pc: {}, unknown opcode {}", pc, o),
            UnknownParameterType(pc, t) => write!(f, "pc: {}, unknown parameter type {}", pc, t),
            InvalidParameterType(pc, t, operation) => write!(
                f,
                "pc: {}, invalid parameter type {} for operation {}",
                pc, t, operation
            ),
            NegativePosition(pc, opcode, position) => write!(
                f,
                "pc: {}, opcode {} has negative position parameter {}",
                pc, opcode, position
            ),
            MissingInput(pc) => write!(f, "pc: {}, input instruction but no input", pc),
        }
    }
}

impl error::Error for IntcodeError {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

struct Instruction<'a> {
    instruction: i64,
    pc: usize,
    program: &'a mut [i64],
}

impl<'a> Instruction<'a> {
    fn new(instruction: i64, pc: usize, program: &'a mut [i64]) -> Instruction {
        return Instruction {
            instruction: instruction,
            pc: pc,
            program: program,
        };
    }

    fn opcode(&self) -> i64 {
        self.instruction % 100
    }

    fn parameter_type(&self, n: u32) -> i64 {
        let parameters = self.instruction / 100;
        (parameters % 10i64.pow(n + 1)) / 10i64.pow(n)
    }

    fn parameter_index(&self, n: u32) -> usize {
        self.pc + 1 + n as usize
    }

    fn intcode_index(&self, i: i64) -> result::Result<usize, IntcodeError> {
        match usize::try_from(i) {
            Err(_) => Err(IntcodeError::NegativePosition(self.pc, self.opcode(), i)),
            Ok(v) => Ok(v),
        }
    }

    fn parameter(&self, n: u32) -> result::Result<i64, IntcodeError> {
        let parameter_type = self.parameter_type(n);
        match parameter_type {
            // position
            0 => {
                let idx = self.intcode_index(self.program[self.parameter_index(n)])?;
                Ok(self.program[idx])
            }
            // immediate
            1 => Ok(self.program[self.parameter_index(n)]),
            _ => Err(IntcodeError::UnknownParameterType(self.pc, parameter_type)),
        }
    }

    fn assign(&'a mut self, n: u32) -> result::Result<&'a mut i64, IntcodeError> {
        let parameter_type = self.parameter_type(n);
        match parameter_type {
            // position
            0 => {
                let idx = self.intcode_index(self.program[self.parameter_index(n)])?;
                Ok(&mut self.program[idx])
            }
            // immediate not supported!
            1 => Err(IntcodeError::InvalidParameterType(
                self.pc,
                parameter_type,
                "assign",
            )),
            _ => Err(IntcodeError::UnknownParameterType(self.pc, parameter_type)),
        }
    }
}

pub struct Machine {
    pc: usize,
    // a machine should own its program, so that it can be re-executed
    // with different inputs without lifetime management.
    program: Vec<i64>,
}

impl Machine {
    pub fn new(program: &[i64]) -> Self {
        Machine {
            pc: 0,
            program: program.to_vec(),
        }
    }

    fn consume_parameters(&mut self, parameters: usize) {
        self.pc += 1 + parameters;
    }

    pub fn execute<'a, I>(&mut self, mut input: I) -> result::Result<Option<i64>, IntcodeError>
    where I: iter::Iterator<Item = &'a i64> {
        loop {
            let mut instruction = Instruction::new(self.program[self.pc], self.pc, &mut self.program);
            match instruction.opcode() {
                // add
                1 => {
                    let res = instruction.parameter(0)? + instruction.parameter(1)?;
                    *instruction.assign(2)? = res;
                    self.consume_parameters(3);
                }
                // multiply
                2 => {
                    *instruction.assign(2)? =
                        instruction.parameter(0)? * instruction.parameter(1)?;
                    self.consume_parameters(3);
                }
                // input
                3 => {
                    let input = match input.next() {
                        Some(input) => *input,
                        None => return Err(IntcodeError::MissingInput(self.pc)),
                    };
                    *instruction.assign(0)? = input;
                    self.consume_parameters(1);
                }
                // output
                4 => {
                    let output = instruction.parameter(0)?;
                    self.consume_parameters(1);
                    return Ok(Some(output));
                }
                // jump-if-true
                5 => {
                    if instruction.parameter(0)? != 0 {
                        self.pc = instruction.intcode_index(instruction.parameter(1)?)?;
                    } else {
                        self.consume_parameters(2);
                    }
                }
                // jump-if-false
                6 => {
                    if instruction.parameter(0)? == 0 {
                        self.pc = instruction.intcode_index(instruction.parameter(1)?)?;
                    } else {
                        self.consume_parameters(2);
                    }
                }
                // less than
                7 => {
                    *instruction.assign(2)? =
                        if instruction.parameter(0)? < instruction.parameter(1)? {
                            1
                        } else {
                            0
                        };
                    self.consume_parameters(3);
                }
                // equals
                8 => {
                    *instruction.assign(2)? =
                        if instruction.parameter(0)? == instruction.parameter(1)? {
                            1
                        } else {
                            0
                        };
                    self.consume_parameters(3);
                }
                99 => return Ok(None),
                _ => return Err(IntcodeError::UnknownOpcode(self.pc, instruction.opcode())),
            }
        }
    }
}

pub fn parse_program(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

pub fn execute<'a>(program: &'a mut [i64]) -> AllOutputResult {
    execute_with_input(program, &[])
}

pub fn execute_with_input<'a>(program: &'a mut [i64], input: &'a [i64]) -> AllOutputResult {
    let mut input = input.iter();
    let mut machine = Machine::new(program);
    let mut output = vec![];
    loop {
        match machine.execute(&mut input)? {
            Some(o) => output.push(o),
            None => {
                // copy the program back into the slice so tests can
                // inspect it.
                program.copy_from_slice(&machine.program);
                return Ok(output)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        assert_eq!(parse_program("1,0,0,0,99"), vec![1, 0, 0, 0, 99])
    }

}
