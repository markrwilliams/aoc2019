#[aoc_generator(day2)]
fn parse_program(input: &str) -> Vec<usize> {
    input.split(",").map(|i| { i.parse::<usize>().unwrap() }).collect()
}

fn parse_parameters(ops: &[usize]) -> (usize, usize, usize) {
    match ops {
        [left, right, result] => (*left, *right, *result),
        _ => panic!("need 3 parameters")
    }
}

fn evaluate(program: &[usize]) -> Vec<usize> {
    let mut executable = program.to_vec();
    for i in (0..executable.len()).step_by(4) {
        match executable[i] {
            1 => {
                let (left, right, result) = parse_parameters(&executable[i+1..i+4]);
                executable[result] = executable[left] + executable[right];
            },
            2 => {
                let (left, right, result) = parse_parameters(&executable[i+1..i+4]);
                executable[result] = executable[left] * executable[right];
            },
            99 => {
                break
            }
            _ => {
                panic!("no good")
            }
        }
    }
    executable
}

#[aoc(day2, part1)]
fn day2_part1(program: &[usize]) -> usize {
    let mut program = program.to_vec();
    program[1] = 12;
    program[2] = 2;
    evaluate(&program)[0]
}

#[aoc(day2, part2)]
fn day2_part2(program: &[usize]) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = program.to_vec();
            program[1] = noun;
            program[2] = verb;
            if evaluate(&program)[0] == 19690720 {
                return 100 * noun + verb
            }
        }
    }
    0
}
