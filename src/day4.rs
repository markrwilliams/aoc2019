#[aoc_generator(day4)]
fn parse_range(input: &str) -> Result<(usize, usize), std::num::ParseIntError> {
    let upper_lower: Vec<usize> = match input
        .split("-")
        .map(|line| line.parse::<usize>())
        .collect() {
            Ok(vec) => vec,
            Err(e) => return Err(e)
        };
    return Ok((upper_lower[0], upper_lower[1]));
}

fn is_password<F>(input: &usize, counter: F) -> bool
                  where F: Fn(&u8) -> bool {
    let inputs: Vec<char> = input.to_string().chars().collect();
    let mut unique: [u8; 10] = [0; 10];
    for c in &inputs {
        unique[c.to_digit(10).unwrap() as usize] += 1;
    }
    let mut sinputs = inputs.clone();
    sinputs.sort();
    unique.iter().any(counter) && inputs == sinputs
}

#[aoc(day4, part1)]
fn day4_part1((lower, upper): &(usize, usize)) -> usize {
    (*lower..*upper)
        .filter(|p| is_password(p, |count| *count > 1))
        .count()
}

#[aoc(day4, part2)]
fn day4_part2((lower, upper): &(usize, usize)) -> usize {
    (*lower..*upper)
        .filter(|p| is_password(p, |count| *count == 2 ))
        .count()
}
