use crate::intcode;

#[aoc_generator(day5)]
fn parse_program(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|i| i.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day5, part1)]
fn day5_part1(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program = program.to_vec();
    let outputs = intcode::execute_with_input(&mut program, &[1])?;
    for output in outputs.iter() {
        println!("output: {}", output);
    }
    Ok(outputs[outputs.len() - 1])
}

#[aoc(day5, part2)]
fn day5_part2(program: &[i64]) -> Result<i64, Box<dyn std::error::Error>> {
    let mut program = program.to_vec();
    let outputs = intcode::execute_with_input(&mut program, &[5])?;
    for output in outputs.iter() {
        println!("output: {}", output);
    }
    Ok(outputs[outputs.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut program: Vec<i64>, expected: Vec<i64>) {
        intcode::execute(&mut program).expect("success");
        assert_eq!(program, expected);
    }

    fn check_input_output(mut program: Vec<i64>, input: Vec<i64>, expected: Vec<i64>) {
        let output = intcode::execute_with_input(&mut program, &input).expect("failure");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_program() {
        assert_eq!(parse_program("1,0,0,0,99"), vec![1, 0, 0, 0, 99])
    }

    #[test]
    fn test_example_1() {
        check(vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn test_example_2() {
        check(vec![1101, 100, -1, 4, 0], vec![1101, 100, -1, 4, 99])
    }

    fn check_equal(is_8: Vec<i64>) {
        check_input_output(is_8.clone(), vec![8], vec![1]);
        check_input_output(is_8.clone(), vec![7], vec![0]);
    }

    fn check_less_than(less_than_8: Vec<i64>) {
        check_input_output(less_than_8.clone(), vec![8], vec![0]);
        check_input_output(less_than_8.clone(), vec![7], vec![1]);
    }

    fn check_jumps(jumps: Vec<i64>) {
        check_input_output(jumps.clone(), vec![0], vec![0]);
        check_input_output(jumps.clone(), vec![1], vec![1]);
        check_input_output(jumps.clone(), vec![2], vec![1]);
    }

    #[test]
    fn test_equal() {
        check_equal(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        check_equal(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    }

    #[test]
    fn test_less_than() {
        check_less_than(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        check_less_than(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    }

    #[test]
    fn test_jumps() {
        check_jumps(vec![
            3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9,
        ]);
        check_jumps(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
    }

    #[test]
    fn test_big_program() {
        let big_program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        // The above example program uses an input instruction to ask
        // for a single number. The program will then output 999 if
        // the input value is below 8
        for below in 0..8 {
            check_input_output(big_program.clone(), vec![below], vec![999])
        }

        // output 1000 if the input value
        // is equal to 8
        check_input_output(big_program.clone(), vec![8], vec![1000]);

        // or output 1001 if the input value is greater
        // than 8.
        for greater in 9..100 {
            check_input_output(big_program.clone(), vec![greater], vec![1001]);
        }
    }
}
