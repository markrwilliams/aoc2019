use crate::intcode;

#[aoc_generator(day2)]
fn parse_program(input: &str) -> Vec<i64> {
    intcode::parse_program(input)
}

#[aoc(day2, part1)]
fn day2_part1(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program = program.to_vec();
    program[1] = 12;
    program[2] = 2;
    intcode::execute(&mut program)?;
    Ok(program[0])
}

#[aoc(day2, part2)]
fn day2_part2(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = program.to_vec();
            program[1] = noun;
            program[2] = verb;
            intcode::execute(&mut program)?;
            if program[0] == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(Box::new(intcode::IntcodeError::UnknownError))
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), intcode::IntcodeError>;

    fn execute(program: Vec<i64>) -> Result<Vec<i64>, intcode::IntcodeError> {
        let mut executed = program.clone();
        intcode::execute(&mut executed)?;
        Ok(executed)
    }

    #[test]
    fn test_example_1() -> TestResult {
        assert_eq!(execute(vec![1, 0, 0, 0, 99])?, vec![2, 0, 0, 0, 99],);
        Ok(())
    }

    #[test]
    fn test_example_2() -> TestResult {
        assert_eq!(execute(vec![2, 3, 0, 3, 99])?, vec![2, 3, 0, 6, 99]);
        Ok(())
    }

    #[test]
    fn test_example_3() -> TestResult {
        assert_eq!(
            execute(vec![2, 4, 4, 5, 99, 0])?,
            vec![2, 4, 4, 5, 99, 9801]
        );
        Ok(())
    }

    #[test]
    fn test_example_4() -> TestResult {
        assert_eq!(
            execute(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])?,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
        Ok(())
    }
}
