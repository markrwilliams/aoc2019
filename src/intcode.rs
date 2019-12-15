use std::error;
use std::result;
use std::fmt;
use std::convert::TryFrom;
use std::iter;
use std::slice;

pub type Result = std::result::Result<Vec<i64>, IntcodeError>;

#[derive(Debug)]
pub enum IntcodeError {
    UnknownError,
    UnknownOpcode(usize, i64),
    InvalidParameterType(usize, i64, &'static str),
    UnknownParameterType(usize, i64),
    NegativePosition(usize, i64, i64),
    MissingInput(usize)
}

impl fmt::Display for IntcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use IntcodeError::*;
        match self {
            UnknownError => write!(f, "unknown error"),
            UnknownOpcode(pc, o) => write!(f, "pc: {}, unknown opcode {}", pc, o),
            UnknownParameterType(pc, t) => write!(f, "pc: {}, unknown parameter type {}", pc, t),
            InvalidParameterType(pc, t, operation) => write!(f, "pc: {}, invalid parameter type {} for operation {}", pc, t, operation),
            NegativePosition(pc, opcode, position) => write!(f, "pc: {}, opcode {} has negative position parameter {}", pc, opcode, position),
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
            Err(_) => Err(IntcodeError::NegativePosition(
                self.pc,
                self.opcode(),
                i,
            )),
            Ok(v) => Ok(v)
        }
    }

    fn parameter(&self, n: u32) -> result::Result<i64, IntcodeError> {
        let parameter_type = self.parameter_type(n);
        match parameter_type {
            // position
            0 => {
                let idx = self.intcode_index(self.program[self.parameter_index(n)])?;
                Ok(self.program[idx])
            },
            // immediate
            1 => {
                Ok(self.program[self.parameter_index(n)])
            }
            _ => Err(IntcodeError::UnknownParameterType(self.pc, parameter_type))
        }
    }

    fn assign(&'a mut self, n: u32) -> result::Result<&'a mut i64, IntcodeError> {
        let parameter_type = self.parameter_type(n);
        match parameter_type {
            // position
            0 => {
                let idx = self.intcode_index(self.program[self.parameter_index(n)])?;
                Ok(&mut self.program[idx])
            },
            // immediate not supported!
            1 => Err(IntcodeError::InvalidParameterType(self.pc, parameter_type, "assign")),
            _ => Err(IntcodeError::UnknownParameterType(self.pc, parameter_type))
        }
    }
}

struct Machine<'a, I: iter::Iterator<Item = &'a i64>> {
    pc: usize,
    program: &'a mut [i64],
    input: I,
}

impl<'a> Machine<'a, slice::Iter<'a, i64>> {
    fn new(program: &'a mut [i64], input: &'a [i64]) -> Self {
        Machine {
            pc: 0,
            program: program,
            input: input.iter(),
        }
    }

    fn consume_parameters(&mut self, parameters: usize) {
        self.pc += 1 + parameters;
    }

    fn execute(&'a mut self) -> Result {
        let mut output: Vec<i64> = vec![];
        loop {
            let mut instruction = Instruction::new(self.program[self.pc], self.pc, self.program);
            match instruction.opcode() {
                // add
                1 => {
                    *instruction.assign(2)? =
                        instruction.parameter(0)? + instruction.parameter(1)?;
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
                    let input = match self.input.next() {
                        Some(input) => *input,
                        None => return Err(IntcodeError::MissingInput(self.pc)),
                    };
                    *instruction.assign(0)? = input;
                    self.consume_parameters(1);
                }
                // output
                4 => {
                    output.push(instruction.parameter(0)?);
                    self.consume_parameters(1);
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
                    *instruction.assign(2)? = if instruction.parameter(0)? < instruction.parameter(1)? {
                        1
                    } else {
                        0
                    };
                    self.consume_parameters(3);
                }
                // equals
                8 => {
                    *instruction.assign(2)? = if instruction.parameter(0)? == instruction.parameter(1)? {
                        1
                    } else {
                        0
                    };
                    self.consume_parameters(3);
                }
                99 => return Ok(output),
                _ => return Err(IntcodeError::UnknownOpcode(self.pc, instruction.opcode())),
            }
        }
    }
}

pub fn execute<'a>(program: &'a mut [i64]) -> Result {
    Machine::new(program, &[]).execute()
}

pub fn execute_with_input<'a>(program: &'a mut [i64], input: &'a [i64]) -> Result {
    Machine::new(program, input).execute()
}
