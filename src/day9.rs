use crate::intcode;

#[aoc_generator(day9)]
fn parse_program(input: &str) -> Vec<i64> {
    intcode::parse_program(input)
}

#[aoc(day9, part1)]
fn day9_part1(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program = program.to_vec();
    let outputs = intcode::execute_with_input(&mut program, &[1])?;
    for output in outputs.iter() {
        println!("output: {}", output);
    }
    Ok(outputs[outputs.len() - 1])
}

#[aoc(day9, part2)]
fn day9_part2(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program = program.to_vec();
    let outputs = intcode::execute_with_input(&mut program, &[2])?;
    for output in outputs.iter() {
        println!("output: {}", output);
    }
    Ok(outputs[outputs.len() - 1])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative() {
        let mut relative = vec![109,3,204,-1,99];
        let output = intcode::execute(&mut relative).expect("execute");
        assert_eq!(output, vec![204]);
    }

    #[test]
    fn test_quine() {
        let mut quine = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let output = intcode::execute(&mut quine).expect("execute");
        assert_eq!(output, quine);
    }

    #[test]
    fn test_16_digit_number() {
        let mut sixteen = vec![1102,34915192,34915192,7,4,7,99,0];
        let output = intcode::execute(&mut sixteen).expect("execute");
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].to_string().len(), 16);
    }

    #[test]
    fn test_middle_number() {
        let mut middle = vec![104,1125899906842624,99];
        let output = intcode::execute(&mut middle).expect("execute");
        assert_eq!(output, vec![middle[1]]);
    }
}
