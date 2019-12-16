use crate::intcode;

#[aoc_generator(day7)]
fn parse_program(input: &str) -> Vec<i64> {
    intcode::parse_program(input)
}

struct Amplifier {
    machine: intcode::Machine,
    phase: i64,
}

impl Amplifier {
    fn new(program: &[i64], phase: i64) -> Self {
        Amplifier{
            machine: intcode::Machine::new(program),
            phase: phase,
        }
    }

    fn amplify(&mut self, input: i64) -> Result<i64, intcode::IntcodeError> {
        let input = [self.phase, input];
        match self.machine.execute(input.iter())? {
            Some(o) => Ok(o),
            None => Err(intcode::IntcodeError::UnknownError),
        }
    }

    fn reamplify(&mut self, input: i64) -> intcode::OutputResult {
        let input = [input];
        match self.machine.execute(input.iter()) {
            Ok(output) => Ok(output),
            Err(e) => Err(e),
        }
    }
}

fn series<'a>(program: &[i64], phases: &'a [i64]) -> Vec<Amplifier> {
    phases
        .iter()
        // to_vec copies the program
        .map(|phase| Amplifier::new(program, *phase))
        .collect()
}

fn execute(series: &mut [Amplifier]) -> Result<i64, intcode::IntcodeError> {
    series.iter_mut().try_fold(0, |input, amplifier| amplifier.amplify(input))
}

fn feedback(series: &mut [Amplifier]) -> Result<i64, intcode::IntcodeError> {
    let mut input = execute(series)?;
    let mut i = 0;
    loop {
        let amplifier = &mut series[i];
        let output = amplifier.reamplify(input)?;
        if output.is_none() {
            return Ok(input);
        }
        input = output.unwrap();
        i = (i + 1) % series.len();
    }
}

// so inefficient!
pub fn permutations(elements: Vec<i64>) -> Vec<Vec<i64>> {
    if elements.len() == 0 {
        vec![vec![]]
    } else {
        let mut perms = vec![];
        for i in 0..elements.len() {
            let element = elements[i];
            let mut without_element = elements.clone();
            without_element.remove(i);
            for partial in permutations(without_element).iter() {
                let mut perm = partial.clone();
                perm.push(element);
                perms.push(perm);
            }
        }
        perms
    }
}

#[aoc(day7, part1)]
fn day7_part1(program: &[i64]) -> i64 {
    let phases = vec![0, 1, 2, 3, 4];
    permutations(phases)
        .iter()
        .map(|phases_permutation| {
            let mut amps = series(program, &phases_permutation);
            execute(&mut amps).expect("oops")
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn day7_part2(program: &[i64]) -> i64 {
    let phases = vec![5, 6, 7, 8, 9];
    permutations(phases)
        .iter()
        .map(|phases_permutation| {
            let mut amps = series(program, &phases_permutation);
            feedback(&mut amps).expect("oops")
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutations() {
        assert_eq!(
            permutations(vec![0, 1, 2]),
            vec![
                vec![2, 1, 0],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![0, 1, 2],
            ]
        )
    }

    fn check_execute(program: Vec<i64>, phases: Vec<i64>, max: i64) {
        let mut amps = series(&program, &phases);
        let res = execute(&mut amps).expect("failure");
        assert_eq!(res, max);
    }

    #[test]
    fn test_example_1() {
        check_execute(
            vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0],
            vec![4,3,2,1,0],
            43210,
        )
    }

    #[test]
    fn test_example_2() {
        check_execute(
            vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0],
            vec![0,1,2,3,4],
            54321,
        )
    }

    #[test]
    fn test_example_3() {
        check_execute(
            vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0],
            vec![1,0,4,3,2],
            65210,
        )
    }

    fn check_feedback(program: Vec<i64>, phases: Vec<i64>, max: i64) {
        let mut amps = series(&program, &phases);
        let res = feedback(&mut amps).expect("failure");
        assert_eq!(res, max);
    }

    #[test]
    fn test_example_4() {
        check_feedback(
            vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5],
            vec![9,8,7,6,5],
            139629729,
        )
    }

    #[test]
    fn test_example_5() {
        check_feedback(
            vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10],
            vec![9,7,8,5,6],
            18216,
        )
    }
}
